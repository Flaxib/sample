// this module adds some functionality based on the required implementations
// here like: `LinkedList::pop_back` or `Clone for LinkedList<T>`
// You are free to use anything in it, but it's mainly for the test framework.
mod pre_implemented;

use std::alloc;
// use std::alloc::{alloc, dealloc, Layout};
use std::ptr;
use std::ptr::NonNull;

// use std::vec::Vec;

pub struct LinkedList<T> {
    // content: NonNull<T>,
    // is_initialised: bool,
    // previous: *mut LinkedList<T>,
    // next: *mut LinkedList<T>,
    node: *mut LinkedListNode<T>,
    _marker: std::marker::PhantomData<T>,
}

struct LinkedListNode<T> {
    content: *mut T,
    previous: *mut LinkedListNode<T>,
    next: *mut LinkedListNode<T>,
    _marker: std::marker::PhantomData<T>,
}

pub struct Cursor<'a, T> {
    node: &'a mut LinkedListNode<T>,
    root: &'a mut LinkedList<T>,
    _marker: std::marker::PhantomData<&'a mut T>,
}

pub struct Iter<'a, T>(std::marker::PhantomData<&'a T>);

// impl<T> LinkedListNode<T> {
//     pub fn new() -> Self {
//         LinkedListNode::<T> {
//             content: ptr::null::<*mut T>() as *mut T,
//             previous: ptr::null::<*mut LinkedListNode<T>>() as *mut LinkedListNode<T>,
//             next: ptr::null::<*mut LinkedListNode<T>>() as *mut LinkedListNode<T>,
//             _marker: std::marker::PhantomData,
//         }
//     }
// }

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        let new_layout = alloc::Layout::new::<LinkedListNode<T>>();
        let new_ptr = unsafe { alloc::alloc(new_layout) as *mut LinkedListNode<T> };

        if NonNull::new(new_ptr) == None {
            alloc::handle_alloc_error(new_layout);
        }

        unsafe {
            (*new_ptr).content = ptr::null::<*mut T>() as *mut T;
            (*new_ptr).previous = ptr::null::<*mut LinkedListNode<T>>() as *mut LinkedListNode<T>;
            (*new_ptr).next = ptr::null::<*mut LinkedListNode<T>>() as *mut LinkedListNode<T>;
        }

        LinkedList::<T> {
            // node: ptr::null::<*mut LinkedListNode<T>>() as *mut LinkedListNode<T>,
            node: new_ptr,
            _marker: std::marker::PhantomData,
        }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for LinkedList)
    pub fn is_empty(&self) -> bool {
        self.node.is_null() || unsafe { (*self.node).content.is_null() }
    }

    pub fn len(&self) -> usize {
        let mut len = 0;

        if self.node.is_null() {
            return 0;
        }
        unsafe {
            let mut current_node = &*self.node;

            while !current_node.next.is_null() {
                current_node = &*current_node.next;
                if !current_node.content.is_null() {
                    len += 1;
                } else {
                    break;
                }
            }
            current_node = &*self.node;
            while !current_node.previous.is_null() {
                current_node = &*current_node.previous;
                if !current_node.content.is_null() {
                    len += 1;
                } else {
                    break;
                }
            }
            if !current_node.content.is_null() {
                len += 1;
            }
        }
        len
    }

    /// Return a cursor positioned on the front element
    pub fn cursor_front(&mut self) -> Cursor<'_, T> {
        if self.node.is_null() {
            panic!("Initial node is not allocated")
        }

        unsafe {
            let mut current_node = &mut *self.node;

            while !current_node.previous.is_null() {
                current_node = &mut *current_node.previous;

                if (*self.node).content.is_null() {
                    if !(*self.node).next.is_null() {
                        panic!("next case: A node in the middle of the list is not used");
                    }

                    alloc::dealloc(
                        self.node as *mut u8,
                        alloc::Layout::new::<LinkedListNode<T>>(),
                    );
                    self.node = current_node;
                }
            }

            Cursor {
                node: current_node,
                root: self,
                _marker: std::marker::PhantomData,
            }
        }
    }

    /// Return a cursor positioned on the back element
    pub fn cursor_back(&mut self) -> Cursor<'_, T> {
        if self.node.is_null() {
            panic!("Initial node is not allocated")
        }

        unsafe {
            let mut current_node = &mut *self.node;

            while !current_node.next.is_null() {
                current_node = &mut *current_node.next;

                if (*self.node).content.is_null() {
                    if !(*self.node).previous.is_null() {
                        panic!("previous case: A node in the middle of the list is not used");
                    }

                    alloc::dealloc(
                        self.node as *mut u8,
                        alloc::Layout::new::<LinkedListNode<T>>(),
                    );
                    self.node = current_node;
                }
            }

            Cursor {
                node: current_node,
                root: self,
                _marker: std::marker::PhantomData,
            }
        }
    }

    /// Return an iterator that moves from front to back
    pub fn iter(&self) -> Iter<'_, T> {
        unimplemented!()
    }
}

// the cursor is expected to act as if it is at the position of an element
// and it also has to work with and be able to insert into an empty list.
impl<T> Cursor<'_, T> {
    /// Take a mutable reference to the current element
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        if self.node.content.is_null() {
            None
        } else {
            Some(unsafe { &mut *self.node.content })
        }
    }

    /// Move one position forward (towards the back) and
    /// return a reference to the new position
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&mut T> {
        if self.node.next.is_null() {
            None
        } else {
            unsafe {
                self.node = &mut *self.node.next;
                // Some(self.node.content.as_mut())
                self.peek_mut()
            }
        }
    }

    /// Move one position backward (towards the front) and
    /// return a reference to the new position
    pub fn prev(&mut self) -> Option<&mut T> {
        if self.node.previous.is_null() {
            None
        } else {
            unsafe {
                self.node = &mut *self.node.previous;
                // Some(&mut *self.node.content)
                // Some(self.node.content.as_mut())
                self.peek_mut()
            }
        }
    }

    /// Remove and return the element at the current position and move the cursor
    /// to the neighboring element that's closest to the back. This can be
    /// either the next or previous position.
    pub fn take(&mut self) -> Option<T> {
        if self.node.content.is_null() && !self.node.previous.is_null() && !self.node.next.is_null()
        {
            panic!("Current node is not initialised but neighboring nodes exist.");
        } else if self.node.content.is_null()
            && self.node.previous.is_null()
            && !self.node.next.is_null()
        {
            // Go to the next node
            self.next();

            // Drop the whole node, value involved
            drop(self.node.previous);
            self.node.previous = ptr::null::<*mut LinkedListNode<T>>() as *mut LinkedListNode<T>;

            self.root.node = self.node;

            None
        } else if self.node.content.is_null()
            && !self.node.previous.is_null()
            && self.node.next.is_null()
        {
            // Go to the previous node
            self.prev();

            // Drop the whole node, value involved
            drop(self.node.next);
            self.node.next = ptr::null::<*mut LinkedListNode<T>>() as *mut LinkedListNode<T>;

            self.root.node = self.node;

            None
        } else if self.node.content.is_null()
            && self.node.previous.is_null()
            && self.node.next.is_null()
        {
            None
        } else if !self.node.content.is_null()
            && !self.node.previous.is_null()
            && !self.node.next.is_null()
        {
            // Link remaining nodes
            unsafe {
                (*self.node.previous).next = self.node.next;
                (*self.node.next).previous = self.node.previous;
            }

            unsafe {
                // Retrieve value before drop
                let res = ptr::read(self.node.content);

                alloc::dealloc(self.node.content as *mut u8, alloc::Layout::new::<T>());
                self.node.content = ptr::null::<*mut T>() as *mut T;

                // Go to the next node
                self.next();

                // Drop the whole node, value involved
                drop(self.node.previous);
                self.node.previous =
                    ptr::null::<*mut LinkedListNode<T>>() as *mut LinkedListNode<T>;

                self.root.node = self.node;

                Some(res)
            }
        } else if !self.node.content.is_null()
            && self.node.previous.is_null()
            && !self.node.next.is_null()
        {
            unsafe {
                // Retrieve value before drop
                let res = ptr::read(self.node.content);

                alloc::dealloc(self.node.content as *mut u8, alloc::Layout::new::<T>());
                self.node.content = ptr::null::<*mut T>() as *mut T;

                // Go to the next node
                self.next();

                // Drop the whole node, value involved
                drop(self.node.previous);
                self.node.previous =
                    ptr::null::<*mut LinkedListNode<T>>() as *mut LinkedListNode<T>;

                self.root.node = self.node;

                Some(res)
            }
        } else if !self.node.content.is_null()
            && !self.node.previous.is_null()
            && self.node.next.is_null()
        {
            // self.node.content.is_null() = false;

            unsafe {
                // Retrieve value before drop
                let res = ptr::read(self.node.content);

                alloc::dealloc(self.node.content as *mut u8, alloc::Layout::new::<T>());
                self.node.content = ptr::null::<*mut T>() as *mut T;

                // Go to the previous node
                self.prev();

                // Drop the whole node, value involved
                drop(self.node.next);
                self.node.next = ptr::null::<*mut LinkedListNode<T>>() as *mut LinkedListNode<T>;

                self.root.node = self.node;

                Some(res)
            }
        } else if !self.node.content.is_null()
            && self.node.previous.is_null()
            && self.node.next.is_null()
        {
            // The initial node is allocated on the stack, so doesn't need to be dropped
            // TODO: This is not necessary the initial node!
            // Some(unsafe { *self.node.content.as_ptr() })
            // self.node.content.is_null() = false;

            unsafe {
                let res = ptr::read(self.node.content);

                alloc::dealloc(self.node.content as *mut u8, alloc::Layout::new::<T>());
                self.node.content = ptr::null::<*mut T>() as *mut T;

                self.root.node = self.node;

                Some(res)
            }
        } else {
            None
        }
    }

    pub fn insert_after(&mut self, element: T) {
        if self.node.content.is_null() {
            // If the list is empty
            let new_layout = alloc::Layout::new::<T>();
            self.node.content = unsafe {
                let new_ptr = alloc::alloc(new_layout) as *mut T;

                if NonNull::new(new_ptr as *mut T) == None {
                    alloc::handle_alloc_error(new_layout);
                }

                *new_ptr = element;

                new_ptr
            };
        } else if self.node.next.is_null() {
            // If the next node doesn't exist
            unsafe {
                let new_layout = alloc::Layout::new::<LinkedListNode<T>>();
                let new_next_node = alloc::alloc(new_layout) as *mut LinkedListNode<T>;

                (*new_next_node).previous = self.node;
                (*new_next_node).next =
                    ptr::null::<*mut LinkedListNode<T>>() as *mut LinkedListNode<T>;

                let new_layout = alloc::Layout::new::<T>();

                (*new_next_node).content = {
                    let new_ptr = alloc::alloc(new_layout) as *mut T;

                    if NonNull::new(new_ptr as *mut T) == None {
                        alloc::handle_alloc_error(new_layout);
                    }

                    *new_ptr = element;

                    new_ptr
                };

                self.node.next = new_next_node;
            }
        } else {
            // General case
            unsafe {
                let new_layout = alloc::Layout::new::<LinkedListNode<T>>();
                let new_next_node = alloc::alloc(new_layout) as *mut LinkedListNode<T>;

                (*new_next_node).previous = self.node;
                (*new_next_node).next = self.node.next;

                let new_layout = alloc::Layout::new::<T>();

                (*new_next_node).content = {
                    let new_ptr = alloc::alloc(new_layout) as *mut T;

                    if NonNull::new(new_ptr as *mut T) == None {
                        alloc::handle_alloc_error(new_layout);
                    }

                    *new_ptr = element;

                    new_ptr
                };

                (*self.node.next).previous = new_next_node;
                self.node.next = new_next_node;
            }
        }
    }

    pub fn insert_before(&mut self, element: T) {
        if self.node.content.is_null() {
            // If the list is empty
            let new_layout = alloc::Layout::new::<T>();
            self.node.content = unsafe {
                let new_ptr = alloc::alloc(new_layout) as *mut T;

                if NonNull::new(new_ptr as *mut T) == None {
                    alloc::handle_alloc_error(new_layout);
                }

                *new_ptr = element;

                new_ptr
            };
        } else if self.node.previous.is_null() {
            // If the previous node doesn't exist
            unsafe {
                let new_layout = alloc::Layout::new::<LinkedListNode<T>>();
                let new_previous_node = alloc::alloc(new_layout) as *mut LinkedListNode<T>;

                (*new_previous_node).next = self.node;
                (*new_previous_node).previous =
                    ptr::null::<*mut LinkedListNode<T>>() as *mut LinkedListNode<T>;

                let new_layout = alloc::Layout::new::<T>();

                (*new_previous_node).content = {
                    let new_ptr = alloc::alloc(new_layout) as *mut T;

                    if NonNull::new(new_ptr as *mut T) == None {
                        alloc::handle_alloc_error(new_layout);
                    }

                    *new_ptr = element;

                    new_ptr
                };

                self.node.previous = new_previous_node;
            }
        } else {
            // General case
            unsafe {
                let new_layout = alloc::Layout::new::<LinkedListNode<T>>();
                let new_previous_node = alloc::alloc(new_layout) as *mut LinkedListNode<T>;

                (*new_previous_node).next = self.node;
                (*new_previous_node).previous = self.node.previous;

                let new_layout = alloc::Layout::new::<T>();

                (*new_previous_node).content = {
                    let new_ptr = alloc::alloc(new_layout) as *mut T;

                    if NonNull::new(new_ptr as *mut T) == None {
                        alloc::handle_alloc_error(new_layout);
                    }

                    *new_ptr = element;

                    new_ptr
                };

                (*self.node.previous).next = new_previous_node;
                self.node.previous = new_previous_node;
            }
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        unimplemented!()
    }
}
