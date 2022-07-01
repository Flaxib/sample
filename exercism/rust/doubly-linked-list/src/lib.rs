// this module adds some functionality based on the required implementations
// here like: `LinkedList::pop_back` or `Clone for LinkedList<T>`
// You are free to use anything in it, but it's mainly for the test framework.
mod pre_implemented;

// Documentation links:
// https://www.youtube.com/watch?v=TJOFSMpJdzg&ab_channel=JonGjengset
//
// Nightly supported only:
// https://youtu.be/TJOFSMpJdzg?t=1154
// #![feature(dropck_eyepatch)] and #[may_dangle] :
//   - tell the compiler the T fields will not be accessed/used in the Drop implementation
//
// Stablely supported:
// https://youtu.be/TJOFSMpJdzg?t=1993
// PhantomData<T> :
//   - assume drop over T field will with be trigger in Drop implementation
//
// https://youtu.be/TJOFSMpJdzg?t=2756
// NonNull<T> :
//   - covariant with T: can use <'static T> where <'a T> is expected
//   - let Rust do the aliasing analisis, whereas it's not possible with `*mut T`
//   - allow the null pointer optimisation
//   (https://stackoverflow.com/questions/46557608/what-is-the-null-pointer-optimization-in-rust)
//
// https://www.youtube.com/watch?v=iVYWDIW71jk&ab_channel=JonGjengset
// https://rust-unofficial.github.io/too-many-lists/sixth.html

use std::ptr::NonNull;

type NodePtr<T> = NonNull<Node<T>>;
type Link<T> = Option<NodePtr<T>>;

pub struct LinkedList<T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,
    _marker: std::marker::PhantomData<T>,
}

// We don't have any shared mutable state nor any thread dependent data.
// Trait bounds for Send/Sync are the same as for Box<T>
unsafe impl<T: Send> Send for LinkedList<T> {}
unsafe impl<T: Sync> Sync for LinkedList<T> {}

struct Node<T> {
    element: T,
    previous: Link<T>,
    next: Link<T>,
}

pub struct Cursor<'a, T> {
    node: Link<T>,
    list: &'a mut LinkedList<T>,
}

pub struct Iter<'a, T> {
    node: Link<T>,
    _marker: std::marker::PhantomData<&'a T>,
}

impl<T> Node<T> {
    pub fn create_linkless(element: T) -> NodePtr<T> {
        unsafe {
            NonNull::new_unchecked(Box::into_raw(Box::new(Self {
                element,
                previous: None,
                next: None,
            })))
        }
    }

    pub unsafe fn link(mut previous: NodePtr<T>, mut next: NodePtr<T>) {
        next.as_mut().previous = Some(previous);
        previous.as_mut().next = Some(NonNull::from(next));
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList::<T> {
            front: None,
            back: None,
            len: 0,
            _marker: std::marker::PhantomData,
        }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for LinkedList)
    pub fn is_empty(&self) -> bool {
        debug_assert!(
            (self.front.is_none() && self.back.is_none())
                || (self.front.is_some() && self.back.is_some())
        );
        self.front.is_none() && self.back.is_none()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    /// Return a cursor positioned on the front element
    pub fn cursor_front(&mut self) -> Cursor<'_, T> {
        Cursor {
            node: self.front,
            list: self,
        }
    }

    /// Return a cursor positioned on the back element
    pub fn cursor_back(&mut self) -> Cursor<'_, T> {
        Cursor {
            node: self.back,
            list: self,
        }
    }

    /// Return an iterator that moves from front to back
    pub fn iter(&self) -> Iter<'_, T> {
        unsafe {
            if self.is_empty() {
                return Iter {
                    node: None,
                    _marker: std::marker::PhantomData,
                };
            }

            let current_node = self.front.unwrap().as_ref();
            Iter {
                node: Some(NonNull::from(current_node)),
                _marker: std::marker::PhantomData,
            }
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut cursor = self.cursor_front();
        while let Some(_) = cursor.take() {}
    }
}

enum BackOrFront {
    Back,
    Front,
}

// the cursor is expected to act as if it is at the position of an element
// and it also has to work with and be able to insert into an empty list.
impl<T> Cursor<'_, T> {
    /// Take a mutable reference to the current element
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.node
            .and_then(|node_ptr| unsafe { Some(&mut (*node_ptr.as_ptr()).element) })
    }

    fn _peek_mut(&mut self, node: Option<NonNull<Node<T>>>) -> Option<&mut T> {
        node.and_then(|node_ptr| {
            self.node = Some(node_ptr);
            self.peek_mut()
        })
    }

    /// Move one position forward (towards the back) and
    /// return a reference to the new position
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&mut T> {
        self.node
            .and_then(|current_ptr| self._peek_mut(unsafe { current_ptr.as_ref().next }))
    }

    /// Move one position backward (towards the front) and
    /// return a reference to the new position
    pub fn prev(&mut self) -> Option<&mut T> {
        self.node
            .and_then(|current_ptr| self._peek_mut(unsafe { current_ptr.as_ref().previous }))
    }

    /// Remove and return the element at the current position and move the cursor
    /// to the neighboring element that's closest to the back. This can be
    /// either the next or previous position.
    pub fn take(&mut self) -> Option<T> {
        if self.list.is_empty() {
            return None;
        }

        let node = unsafe { self.node.unwrap().as_mut() };

        if node.previous.is_some() && node.next.is_some() {
            unsafe {
                // Link remaining nodes
                (*node.previous.unwrap().as_ptr()).next = node.next;
                (*node.next.unwrap().as_ptr()).previous = node.previous;

                // Retrieve address before move
                let node_to_take_address = self.node?.as_ptr();

                // Go to the next node
                self.next();

                if node_to_take_address == self.list.front.unwrap().as_ptr() {
                    self.list.front = self.node;
                };
                if node_to_take_address == self.list.back.unwrap().as_ptr() {
                    self.list.back = self.node;
                };
                self.list.len -= 1;

                Some(Box::from_raw(node_to_take_address).element)
            }
        } else if node.previous.is_none() && node.next.is_some() {
            unsafe {
                // Link remaining nodes
                (*node.next.unwrap().as_ptr()).previous = None;

                // Retrieve address before move
                let node_to_take_address = self.node?.as_ptr();

                // Go to the next node
                self.next();

                if node_to_take_address == self.list.front.unwrap().as_ptr() {
                    self.list.front = self.node;
                };
                if node_to_take_address == self.list.back.unwrap().as_ptr() {
                    self.list.back = self.node;
                };
                self.list.len -= 1;

                Some(Box::from_raw(node_to_take_address).element)
            }
        } else if node.previous.is_some() && node.next.is_none() {
            unsafe {
                // Link remaining nodes
                (*node.previous.unwrap().as_ptr()).next = None;

                // Retrieve address before move
                let node_to_take_address = self.node?.as_ptr();

                // Go to the previous node
                self.prev();

                if node_to_take_address == self.list.front.unwrap().as_ptr() {
                    self.list.front = self.node;
                };
                if node_to_take_address == self.list.back.unwrap().as_ptr() {
                    self.list.back = self.node;
                };
                self.list.len -= 1;

                Some(Box::from_raw(node_to_take_address).element)
            }
        } else {
            unsafe {
                self.list.front = None;
                self.list.back = None;
                self.list.len -= 1;

                Some(Box::from_raw(self.node?.as_ptr()).element)
            }
        }
    }

    pub fn insert_after(&mut self, element: T) {
        let do_insert = unsafe {
            |cursor_node: &mut NodePtr<T>, new_node_to_insert: &mut NodePtr<T>| {
                if let Some(next_node) = cursor_node.as_mut().next {
                    Node::<T>::link(*new_node_to_insert, next_node);
                }
                Node::<T>::link(*cursor_node, *new_node_to_insert);
            }
        };

        self._insert(element, BackOrFront::Back, do_insert);
    }

    pub fn insert_before(&mut self, element: T) {
        let do_insert = unsafe {
            |cursor_node: &mut NodePtr<T>, new_node_to_insert: &mut NodePtr<T>| {
                if let Some(previous_node) = cursor_node.as_mut().previous {
                    Node::<T>::link(previous_node, *new_node_to_insert);
                }
                Node::<T>::link(*new_node_to_insert, *cursor_node);
            }
        };

        self._insert(element, BackOrFront::Front, do_insert);
    }

    fn _insert(
        &mut self,
        element: T,
        back_or_front: BackOrFront,
        do_insert: impl Fn(&mut NodePtr<T>, &mut NodePtr<T>),
    ) {
        let mut new_node_to_insert = Node::<T>::create_linkless(element);

        let mut cursor_node = match self.node {
            Some(node) => node,
            None => {
                self.node = Some(new_node_to_insert);
                self.list.front = self.node;
                self.list.back = self.node;
                self.list.len += 1;
                return;
            }
        };

        do_insert(&mut cursor_node, &mut new_node_to_insert);

        let extremity_node = match back_or_front {
            BackOrFront::Front => &mut self.list.front,
            BackOrFront::Back => &mut self.list.back,
        };

        if *extremity_node == Some(cursor_node) {
            *extremity_node = Some(new_node_to_insert);
        }

        self.list.len += 1;
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if self.node.is_none() {
            return None;
        }

        unsafe {
            let res = &(*self.node.unwrap().as_ptr()).element;
            self.node = (*self.node.unwrap().as_ptr()).next;

            Some(res)
        }
    }
}
