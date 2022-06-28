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

use std::alloc;
use std::ptr;
use std::ptr::NonNull;

type NodePtr<T> = NonNull<Node<T>>;
type Link<T> = Option<NodePtr<T>>;

pub struct LinkedList<T> {
    front: Link<T>,
    back: Link<T>,
    // TODO: add len as field
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
    pub fn create(element: T) -> NodePtr<T> {
        unsafe {
            NonNull::new_unchecked(Box::into_raw(Box::new(Self {
                element,
                previous: None,
                next: None,
            })))
        }
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList::<T> {
            front: None,
            back: None,
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
        let mut len = 0;

        if self.is_empty() {
            return 0;
        }

        len += 1;

        unsafe {
            let mut current_node = self.front.unwrap().as_ref();

            while current_node.next.is_some() {
                len += 1;
                current_node = &*current_node.next.unwrap().as_ptr();
            }
        }
        len
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

// the cursor is expected to act as if it is at the position of an element
// and it also has to work with and be able to insert into an empty list.
impl<T> Cursor<'_, T> {
    /// Take a mutable reference to the current element
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        if self.list.is_empty() {
            return None;
        } else {
            // Some(&mut (*self.node?.as_ptr()).element)
            unsafe { Some(&mut (*self.node?.as_ptr()).element) }
        }
    }

    /// Move one position forward (towards the back) and
    /// return a reference to the new position
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&mut T> {
        if self.list.is_empty() {
            return None;
        }
        let node = unsafe { self.node.unwrap().as_ref() };
        if node.next.is_none() {
            None
        } else {
            self.node = node.next;
            self.peek_mut()
        }
    }

    /// Move one position backward (towards the front) and
    /// return a reference to the new position
    pub fn prev(&mut self) -> Option<&mut T> {
        if self.list.is_empty() {
            return None;
        }
        let node = unsafe { self.node.unwrap().as_ref() };
        if node.previous.is_none() {
            None
        } else {
            self.node = node.previous;
            self.peek_mut()
        }
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

                // Retrieve value before drop
                let res = ptr::read(&node.element);

                let to_deallocate = node as *mut Node<T>;

                // Go to the next node
                self.next();

                // Drop the whole node, value involved
                alloc::dealloc(to_deallocate as *mut u8, alloc::Layout::new::<Node<T>>());

                if to_deallocate == self.list.front.unwrap().as_ptr() {
                    self.list.front = Some(NonNull::from(self.node.unwrap().as_ref()));
                };
                if to_deallocate == self.list.back.unwrap().as_ptr() {
                    self.list.back = Some(NonNull::from(self.node.unwrap().as_ref()));
                };

                Some(res)
            }
        } else if node.previous.is_none() && node.next.is_some() {
            unsafe {
                // Retrieve value before drop
                let res = ptr::read(&node.element);

                let to_deallocate = node as *mut Node<T>;

                // Go to the next node
                self.next();

                // Drop the whole node, value involved
                alloc::dealloc(to_deallocate as *mut u8, alloc::Layout::new::<Node<T>>());

                self.node.unwrap().as_mut().previous = None;

                if to_deallocate == self.list.front.unwrap().as_ptr() {
                    self.list.front = Some(NonNull::from(self.node.unwrap().as_ref()));
                };
                if to_deallocate == self.list.back.unwrap().as_ptr() {
                    self.list.back = Some(NonNull::from(self.node.unwrap().as_ref()));
                };

                Some(res)
            }
        } else if node.previous.is_some() && node.next.is_none() {
            // node.content.is_null() = false;

            unsafe {
                // Retrieve value before drop
                let res = ptr::read(&node.element);

                let to_deallocate = node as *mut Node<T>;

                // Go to the previous node
                self.prev();

                // Drop the whole node, value involved
                alloc::dealloc(to_deallocate as *mut u8, alloc::Layout::new::<Node<T>>());

                self.node.unwrap().as_mut().next = None;

                if to_deallocate == self.list.front.unwrap().as_ptr() {
                    self.list.front = Some(NonNull::from(self.node.unwrap().as_ref()));
                };
                if to_deallocate == self.list.back.unwrap().as_ptr() {
                    self.list.back = Some(NonNull::from(self.node.unwrap().as_ref()));
                };

                Some(res)
            }
        } else {
            unsafe {
                let res = ptr::read(&node.element);

                // Deallocate the only remaining node
                alloc::dealloc(
                    self.list.front.unwrap().as_ptr() as *mut u8,
                    alloc::Layout::new::<Node<T>>(),
                );
                self.list.front = None;
                self.list.back = None;

                Some(res)
            }
        }
    }

    pub fn insert_after(&mut self, element: T) {
        if self.list.is_empty() {
            self.list.front = Some(Node::<T>::create(element));
            self.list.back = self.list.front;
            self.node = self.list.front;
            return;
        }

        let node = unsafe { self.node.unwrap().as_mut() };

        if node.next.is_none() {
            // If the next node doesn't exist
            let new_next_node = Node::<T>::create(element);
            node.next = Some(new_next_node);
            unsafe {
                (*new_next_node.as_ptr()).previous = self.node;
            }
            self.list.back = Some(new_next_node);
        } else {
            // If there is a next node doesn't exist
            let new_next_node = Node::<T>::create(element);
            let existing_next_node = unsafe { &mut *node.next.unwrap().as_ptr() };

            existing_next_node.previous = Some(new_next_node);
            node.next = Some(new_next_node);

            unsafe {
                (*new_next_node.as_ptr()).next = Some(NonNull::from(existing_next_node));
                (*new_next_node.as_ptr()).previous = self.node;
            }
        }
    }

    pub fn insert_before(&mut self, element: T) {
        if self.list.is_empty() {
            self.list.front = Some(Node::<T>::create(element));
            self.list.back = self.list.front;
            self.node = self.list.front;
            return;
        }

        let node = unsafe { self.node.unwrap().as_mut() };

        if node.previous.is_none() {
            // If the previous node doesn't exist
            let new_previous_node = Node::<T>::create(element);
            node.previous = Some(new_previous_node);
            unsafe {
                (*new_previous_node.as_ptr()).next = self.node;
            }
            self.list.front = Some(new_previous_node);
        } else {
            // If there is a previous node
            let new_previous_node = Node::<T>::create(element);
            let mut existing_previous_node = unsafe { &mut *node.previous.unwrap().as_ptr() };

            existing_previous_node.next = Some(new_previous_node);
            node.previous = Some(new_previous_node);

            unsafe {
                (*new_previous_node.as_ptr()).previous =
                    Some(NonNull::from(existing_previous_node));
                (*new_previous_node.as_ptr()).next = self.node;
            }
        }
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
