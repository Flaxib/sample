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
    content: NonNull<T>,
    is_initialised: bool,
    previous: *mut LinkedList<T>,
    next: *mut LinkedList<T>,
    _marker: std::marker::PhantomData<T>,
}

pub struct Cursor<'a, T> {
    node: &'a mut LinkedList<T>,
    _marker: std::marker::PhantomData<&'a mut T>,
}

pub struct Iter<'a, T>(std::marker::PhantomData<&'a T>);

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        let new_layout = alloc::Layout::new::<T>();
        let new_ptr = unsafe { alloc::alloc(new_layout) };

        LinkedList::<T> {
            // content: ptr::null::<*mut T>() as *mut T,
            content: match NonNull::new(new_ptr as *mut T) {
                Some(p) => p,
                None => alloc::handle_alloc_error(new_layout),
            },
            is_initialised: false,
            previous: ptr::null::<*mut LinkedList<T>>() as *mut LinkedList<T>,
            next: ptr::null::<*mut LinkedList<T>>() as *mut LinkedList<T>,
            _marker: std::marker::PhantomData,
        }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for LinkedList)
    pub fn is_empty(&self) -> bool {
        !self.is_initialised && self.previous.is_null() && self.next.is_null()
        // unimplemented!()
    }

    pub fn len(&self) -> usize {
        //  : LinkedList<T>
        let mut current_node = self;
        let mut len = 0;
        while !current_node.next.is_null() {
            unsafe {
                current_node = &*current_node.next;
            }
            if current_node.is_initialised {
                len += 1;
            } else {
                break;
            }
        }
        current_node = self;
        while !current_node.previous.is_null() {
            unsafe {
                current_node = &*current_node.previous;
            }
            if current_node.is_initialised {
                len += 1;
            } else {
                break;
            }
        }
        if self.is_initialised {
            len += 1;
        }
        len
    }

    /// Return a cursor positioned on the front element
    pub fn cursor_front(&mut self) -> Cursor<'_, T> {
        let mut current_node = self;

        if !current_node.is_initialised {
            return Cursor {
                node: current_node,
                _marker: std::marker::PhantomData,
            };
        }

        while !current_node.previous.is_null() {
            unsafe {
                current_node = &mut *current_node.previous;
            }
        }

        Cursor {
            node: current_node,
            _marker: std::marker::PhantomData,
        }
    }

    /// Return a cursor positioned on the back element
    pub fn cursor_back(&mut self) -> Cursor<'_, T> {
        let mut current_node = self;

        if !current_node.is_initialised {
            return Cursor {
                node: current_node,
                _marker: std::marker::PhantomData,
            };
        }

        while !current_node.next.is_null() {
            unsafe {
                current_node = &mut *current_node.next;
            }
        }

        Cursor {
            node: current_node,
            _marker: std::marker::PhantomData,
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
        if !self.node.is_initialised {
            None
        } else {
            Some(unsafe { self.node.content.as_mut() })
        }
    }

    /// Move one position forward (towards the back) and
    /// return a reference to the new position
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&mut T> {
        if self.node.next.is_null() {
            None
        } else {
            // self.node = &mut unsafe { *self.node.next };
            unsafe {
                self.node = &mut *self.node.next;
                Some(self.node.content.as_mut())
            }
            // self.node = self.node.next;
            // Some(&mut unsafe { *self.node.content })
        }
    }

    /// Move one position backward (towards the front) and
    /// return a reference to the new position
    pub fn prev(&mut self) -> Option<&mut T> {
        if self.node.previous.is_null() {
            None
        } else {
            // self.node = &mut unsafe { *self.node.previous }; // as &mut LinkedList<T>;
            // Some(&mut unsafe { *self.node.content }) // as &mut LinkedList<T>)
            unsafe {
                self.node = &mut *self.node.previous;
                // Some(&mut *self.node.content)
                Some(self.node.content.as_mut())
            }
        }
    }

    /// Remove and return the element at the current position and move the cursor
    /// to the neighboring element that's closest to the back. This can be
    /// either the next or previous position.
    pub fn take(&mut self) -> Option<T> {
        if !self.node.is_initialised && !self.node.previous.is_null() && !self.node.next.is_null() {
            panic!("Current node is not initialised but neighboring nodes exist.");
        } else if !self.node.is_initialised
            && self.node.previous.is_null()
            && !self.node.next.is_null()
        {
            // Go to the next node
            self.next();

            // Drop the whole node, value involved
            drop(self.node.previous);
            self.node.previous = ptr::null::<*mut LinkedList<T>>() as *mut LinkedList<T>;
            None
        } else if !self.node.is_initialised
            && !self.node.previous.is_null()
            && self.node.next.is_null()
        {
            // Go to the previous node
            self.prev();

            // Drop the whole node, value involved
            drop(self.node.next);
            self.node.next = ptr::null::<*mut LinkedList<T>>() as *mut LinkedList<T>;
            None
        } else if !self.node.is_initialised
            && self.node.previous.is_null()
            && self.node.next.is_null()
        {
            None
        } else if self.node.is_initialised
            && !self.node.previous.is_null()
            && !self.node.next.is_null()
        {
            // Link remaining nodes
            unsafe {
                (*self.node.previous).next = self.node.next;
                (*self.node.next).previous = self.node.previous;
            }

            self.node.is_initialised = false;

            unsafe {
                // Retrieve value before drop
                let res = ptr::read(self.node.content.as_ptr());

                // Go to the next node
                self.next();

                // Drop the whole node, value involved
                drop(self.node.previous);

                Some(res)
            }
        } else if self.node.is_initialised
            && self.node.previous.is_null()
            && !self.node.next.is_null()
        {
            self.node.is_initialised = false;

            unsafe {
                // Retrieve value before drop
                let res = ptr::read(self.node.content.as_ptr());

                // Go to the next node
                self.next();

                // Drop the whole node, value involved
                drop(self.node.previous);

                self.node.previous = ptr::null::<*mut LinkedList<T>>() as *mut LinkedList<T>;

                Some(res)
            }
        } else if self.node.is_initialised
            && !self.node.previous.is_null()
            && self.node.next.is_null()
        {
            self.node.is_initialised = false;

            unsafe {
                // Retrieve value before drop
                let res = ptr::read(self.node.content.as_ptr());

                // Go to the previous node
                self.prev();

                // Drop the whole node, value involved
                drop(self.node.next);
                self.node.next = ptr::null::<*mut LinkedList<T>>() as *mut LinkedList<T>;
                Some(res)
            }
        } else if self.node.is_initialised
            && self.node.previous.is_null()
            && self.node.next.is_null()
        {
            // The initial node is allocated on the stack, so doesn't need to be dropped
            // TODO: This is not necessary the initial node!
            // Some(unsafe { *self.node.content.as_ptr() })
            self.node.is_initialised = false;
            Some(unsafe { ptr::read(self.node.content.as_ptr()) })
        } else {
            None
        }
    }

    pub fn insert_after(&mut self, element: T) {
        if !self.node.is_initialised {
            // If the list is empty
            let new_layout = alloc::Layout::new::<T>();
            self.node.content = unsafe {
                let new_ptr = alloc::alloc(new_layout) as *mut T;

                match NonNull::new(new_ptr as *mut T) {
                    Some(p) => p,
                    None => alloc::handle_alloc_error(new_layout),
                }
            };

            unsafe {
                *self.node.content.as_ptr() = element;
            }
            self.node.is_initialised = true;
        } else if self.node.next.is_null() {
            // If the next node doesn't exist
            unsafe {
                let new_layout = alloc::Layout::new::<LinkedList<T>>();
                let new_next_node = alloc::alloc(new_layout) as *mut LinkedList<T>;

                (*new_next_node).previous = self.node;

                let new_layout = alloc::Layout::new::<T>();

                (*new_next_node).content = {
                    let new_ptr = alloc::alloc(new_layout) as *mut T;

                    match NonNull::new(new_ptr as *mut T) {
                        Some(p) => p,
                        None => alloc::handle_alloc_error(new_layout),
                    }
                };

                *(*new_next_node).content.as_ptr() = element;

                (*new_next_node).is_initialised = true;
                self.node.next = new_next_node;
            }
        } else {
            // General case
            unsafe {
                let new_layout = alloc::Layout::new::<LinkedList<T>>();
                let new_next_node = alloc::alloc(new_layout) as *mut LinkedList<T>;

                (*new_next_node).previous = self.node;
                (*new_next_node).next = self.node.next;

                let new_layout = alloc::Layout::new::<T>();

                (*new_next_node).content = {
                    let new_ptr = alloc::alloc(new_layout) as *mut T;

                    match NonNull::new(new_ptr as *mut T) {
                        Some(p) => p,
                        None => alloc::handle_alloc_error(new_layout),
                    }
                };

                *(*new_next_node).content.as_ptr() = element;

                (*new_next_node).is_initialised = true;
                (*self.node.next).previous = new_next_node;
                self.node.next = new_next_node;
            }
        }
    }

    pub fn insert_before(&mut self, element: T) {
        if !self.node.is_initialised {
            // If the list is empty
            let new_layout = alloc::Layout::new::<T>();
            self.node.content = unsafe {
                let new_ptr = alloc::alloc(new_layout) as *mut T;

                match NonNull::new(new_ptr as *mut T) {
                    Some(p) => p,
                    None => alloc::handle_alloc_error(new_layout),
                }
            };

            unsafe {
                *self.node.content.as_ptr() = element;
            }
            self.node.is_initialised = true;
        } else if self.node.previous.is_null() {
            // If the previous node doesn't exist
            unsafe {
                let new_layout = alloc::Layout::new::<LinkedList<T>>();
                let new_previous_node = alloc::alloc(new_layout) as *mut LinkedList<T>;

                (*new_previous_node).next = self.node;

                let new_layout = alloc::Layout::new::<T>();

                (*new_previous_node).content = {
                    let new_ptr = alloc::alloc(new_layout) as *mut T;

                    match NonNull::new(new_ptr as *mut T) {
                        Some(p) => p,
                        None => alloc::handle_alloc_error(new_layout),
                    }
                };

                *(*new_previous_node).content.as_ptr() = element;

                (*new_previous_node).is_initialised = true;
                self.node.previous = new_previous_node;
            }
        } else {
            // General case
            unsafe {
                let new_layout = alloc::Layout::new::<LinkedList<T>>();
                let new_previous_node = alloc::alloc(new_layout) as *mut LinkedList<T>;

                (*new_previous_node).next = self.node;
                (*new_previous_node).previous = self.node.previous;

                let new_layout = alloc::Layout::new::<T>();

                (*new_previous_node).content = {
                    let new_ptr = alloc::alloc(new_layout) as *mut T;

                    match NonNull::new(new_ptr as *mut T) {
                        Some(p) => p,
                        None => alloc::handle_alloc_error(new_layout),
                    }
                };

                *(*new_previous_node).content.as_ptr() = element;

                (*new_previous_node).is_initialised = true;
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
