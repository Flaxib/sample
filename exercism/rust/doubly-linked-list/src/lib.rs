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

use std::alloc;
use std::ptr;
use std::ptr::NonNull;

pub struct LinkedList<T> {
    node: *mut LinkedListNode<T>,
    // TODO: is_valid: bool,
    _marker: std::marker::PhantomData<T>,
}

struct LinkedListNode<T> {
    content: NonNull<T>,
    previous: NonNull<LinkedListNode<T>>,
    next: NonNull<LinkedListNode<T>>,
    has_content: bool,
    has_previous: bool,
    has_next: bool,
    _marker: std::marker::PhantomData<T>,
}

pub struct Cursor<'a, T> {
    node: &'a mut LinkedListNode<T>,
    root: &'a mut LinkedList<T>,
    _marker: std::marker::PhantomData<&'a mut T>,
}

pub struct Iter<'a, T> {
    node: *const LinkedListNode<T>,
    is_ended: bool,
    _marker: std::marker::PhantomData<&'a T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        let new_layout = alloc::Layout::new::<LinkedListNode<T>>();
        let new_ptr = unsafe { alloc::alloc(new_layout) as *mut LinkedListNode<T> };

        if NonNull::new(new_ptr) == None {
            alloc::handle_alloc_error(new_layout);
        }

        unsafe {
            (*new_ptr).content = NonNull::dangling();
            (*new_ptr).previous = NonNull::dangling();
            (*new_ptr).next = NonNull::dangling();
            (*new_ptr).has_content = false;
            (*new_ptr).has_previous = false;
            (*new_ptr).has_next = false;
        }

        LinkedList::<T> {
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
        self.node.is_null() || unsafe { !(*self.node).has_content }
    }

    pub fn len(&self) -> usize {
        let mut len = 0;

        if self.node.is_null() {
            return 0;
        }
        unsafe {
            let mut current_node = &*self.node;

            while current_node.has_next {
                current_node = current_node.next.as_ref();
                if current_node.has_content {
                    len += 1;
                } else {
                    break;
                }
            }
            current_node = &*self.node;
            while current_node.has_previous {
                current_node = current_node.previous.as_ref();
                if current_node.has_content {
                    len += 1;
                } else {
                    break;
                }
            }
            if current_node.has_content {
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

            while current_node.has_previous {
                current_node = current_node.previous.as_mut();
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

            while current_node.has_next {
                current_node = current_node.next.as_mut();
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
        unsafe {
            let mut current_node = self.node as *const LinkedListNode<T>;

            if current_node.is_null() {
                return Iter {
                    node: current_node,
                    is_ended: true,
                    _marker: std::marker::PhantomData,
                };
            }

            while (*current_node).has_previous {
                current_node = (*current_node).previous.as_ptr();
            }

            Iter {
                node: current_node,
                is_ended: false,
                _marker: std::marker::PhantomData,
            }
        }
    }
}

// the cursor is expected to act as if it is at the position of an element
// and it also has to work with and be able to insert into an empty list.
impl<T> Cursor<'_, T> {
    /// Take a mutable reference to the current element
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        if !self.node.has_content {
            None
        } else {
            Some(unsafe { self.node.content.as_mut() })
        }
    }

    /// Move one position forward (towards the back) and
    /// return a reference to the new position
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&mut T> {
        if !self.node.has_next {
            None
        } else {
            unsafe {
                self.node = self.node.next.as_mut();
                self.peek_mut()
            }
        }
    }

    /// Move one position backward (towards the front) and
    /// return a reference to the new position
    pub fn prev(&mut self) -> Option<&mut T> {
        if !self.node.has_previous {
            None
        } else {
            unsafe {
                self.node = self.node.previous.as_mut();
                self.peek_mut()
            }
        }
    }

    /// Remove and return the element at the current position and move the cursor
    /// to the neighboring element that's closest to the back. This can be
    /// either the next or previous position.
    pub fn take(&mut self) -> Option<T> {
        if !self.node.has_content && self.node.has_previous && self.node.has_next {
            panic!("Current node is not initialised but neighboring nodes exist.");
        } else if !self.node.has_content && !self.node.has_previous && self.node.has_next {
            let to_deallocate = self.node as *mut LinkedListNode<T>;

            // Go to the next node
            self.next();

            // Drop the whole node, value involved
            // drop(self.node.previous);
            // self.node.previous = ptr::null::<*mut LinkedListNode<T>>() as *mut LinkedListNode<T>;

            unsafe {
                alloc::dealloc(
                    to_deallocate as *mut u8,
                    alloc::Layout::new::<*mut LinkedListNode<T>>(),
                );
            }

            self.node.previous = NonNull::dangling();
            self.node.has_previous = false;

            if to_deallocate == self.root.node {
                self.root.node = self.node;
            }

            None
        } else if !self.node.has_content && self.node.has_previous && !self.node.has_next {
            let to_deallocate = self.node as *mut LinkedListNode<T>;

            // Go to the previous node
            self.prev();

            // Drop the whole node, value involved
            // drop(self.node.next);
            // self.node.next = ptr::null::<*mut LinkedListNode<T>>() as *mut LinkedListNode<T>;

            unsafe {
                alloc::dealloc(
                    to_deallocate as *mut u8,
                    alloc::Layout::new::<*mut LinkedListNode<T>>(),
                );
            }

            self.node.next = NonNull::dangling();
            self.node.has_next = false;

            if to_deallocate == self.root.node {
                self.root.node = self.node;
            };

            None
        } else if !self.node.has_content && !self.node.has_previous && !self.node.has_next {
            None
        } else if self.node.has_content && self.node.has_previous && self.node.has_next {
            unsafe {
                // Link remaining nodes
                (*self.node.previous.as_ptr()).next = self.node.next;
                (*self.node.next.as_ptr()).previous = self.node.previous;

                // Retrieve value before drop
                let res = ptr::read(self.node.content.as_ptr());

                alloc::dealloc(
                    self.node.content.as_ptr() as *mut u8,
                    alloc::Layout::new::<T>(),
                );
                self.node.has_content = false;
                // self.node.content = ptr::null::<*mut T>() as *mut T;

                let to_deallocate = self.node as *mut LinkedListNode<T>;

                // Go to the next node
                self.next();

                // Drop the whole node, value involved
                // drop(self.node.previous);
                // self.node.previous =
                //     ptr::null::<*mut LinkedListNode<T>>() as *mut LinkedListNode<T>;
                alloc::dealloc(
                    to_deallocate as *mut u8,
                    alloc::Layout::new::<*mut LinkedListNode<T>>(),
                );

                if to_deallocate == self.root.node {
                    self.root.node = self.node;
                };

                Some(res)
            }
        } else if self.node.has_content && !self.node.has_previous && self.node.has_next {
            unsafe {
                // Retrieve value before drop
                let res = ptr::read(self.node.content.as_ptr());

                alloc::dealloc(
                    self.node.content.as_ptr() as *mut u8,
                    alloc::Layout::new::<T>(),
                );
                self.node.has_content = false;
                // self.node.content = ptr::null::<*mut T>() as *mut T;

                let to_deallocate = self.node as *mut LinkedListNode<T>;

                // Go to the next node
                self.next();

                // Drop the whole node, value involved
                // drop(self.node.previous);
                // self.node.previous =
                //     ptr::null::<*mut LinkedListNode<T>>() as *mut LinkedListNode<T>;
                alloc::dealloc(
                    to_deallocate as *mut u8,
                    alloc::Layout::new::<*mut LinkedListNode<T>>(),
                );

                self.node.previous = NonNull::dangling();
                self.node.has_previous = false;

                if to_deallocate == self.root.node {
                    self.root.node = self.node;
                };

                Some(res)
            }
        } else if self.node.has_content && self.node.has_previous && !self.node.has_next {
            // self.node.content.is_null() = false;

            unsafe {
                // Retrieve value before drop
                let res = ptr::read(self.node.content.as_ptr());

                alloc::dealloc(
                    self.node.content.as_ptr() as *mut u8,
                    alloc::Layout::new::<T>(),
                );
                self.node.has_content = false;
                // self.node.content = ptr::null::<*mut T>() as *mut T;

                let to_deallocate = self.node as *mut LinkedListNode<T>;

                // Go to the previous node
                self.prev();

                // Drop the whole node, value involved
                // drop(self.node.next);
                // self.node.next = ptr::null::<*mut LinkedListNode<T>>() as *mut LinkedListNode<T>;
                alloc::dealloc(
                    to_deallocate as *mut u8,
                    alloc::Layout::new::<*mut LinkedListNode<T>>(),
                );

                self.node.next = NonNull::dangling();
                self.node.has_next = false;

                if to_deallocate == self.root.node {
                    self.root.node = self.node;
                }

                Some(res)
            }
        } else if self.node.has_content && !self.node.has_previous && !self.node.has_next {
            unsafe {
                let res = ptr::read(self.node.content.as_ptr());

                alloc::dealloc(
                    self.node.content.as_ptr() as *mut u8,
                    alloc::Layout::new::<T>(),
                );
                self.node.has_content = false;
                // self.node.content = ptr::null::<*mut T>() as *mut T;

                // if to_deallocate == self.root.node {
                //     self.root.node = self.node;
                // };

                Some(res)
            }
        } else {
            None
        }
    }

    pub fn insert_after(&mut self, element: T) {
        if !self.node.has_content {
            // If the list is empty
            let new_layout = alloc::Layout::new::<T>();
            self.node.content = unsafe {
                let new_ptr = alloc::alloc(new_layout) as *mut T;

                match NonNull::new(new_ptr as *mut T) {
                    Some(ptr) => {
                        *new_ptr = element;
                        ptr
                    }
                    None => alloc::handle_alloc_error(new_layout),
                }
            };
            self.node.has_content = true;
        } else if !self.node.has_next {
            // If the next node doesn't exist
            unsafe {
                let new_layout = alloc::Layout::new::<LinkedListNode<T>>();
                let new_next_node = alloc::alloc(new_layout) as *mut LinkedListNode<T>;

                (*new_next_node).previous = match NonNull::new(self.node) {
                    Some(ptr) => ptr,
                    None => alloc::handle_alloc_error(new_layout),
                };
                (*new_next_node).has_previous = true;
                (*new_next_node).next = NonNull::dangling();
                (*new_next_node).has_next = false;

                let new_layout = alloc::Layout::new::<T>();

                (*new_next_node).content = {
                    let new_ptr = alloc::alloc(new_layout) as *mut T;

                    match NonNull::new(new_ptr as *mut T) {
                        Some(ptr) => {
                            *new_ptr = element;
                            ptr
                        }
                        None => alloc::handle_alloc_error(new_layout),
                    }
                };
                (*new_next_node).has_content = true;

                self.node.next = match NonNull::new(new_next_node) {
                    Some(ptr) => ptr,
                    None => alloc::handle_alloc_error(new_layout),
                };
                self.node.has_next = true;
            }
        } else {
            // General case
            unsafe {
                let new_layout = alloc::Layout::new::<LinkedListNode<T>>();
                let new_next_node = alloc::alloc(new_layout) as *mut LinkedListNode<T>;

                (*new_next_node).previous = match NonNull::new(self.node) {
                    Some(ptr) => ptr,
                    None => alloc::handle_alloc_error(new_layout),
                };
                (*new_next_node).has_previous = true;
                (*new_next_node).next = self.node.next;
                (*new_next_node).has_next = true;

                let new_layout = alloc::Layout::new::<T>();

                (*new_next_node).content = {
                    let new_ptr = alloc::alloc(new_layout) as *mut T;

                    match NonNull::new(new_ptr as *mut T) {
                        Some(ptr) => {
                            *new_ptr = element;
                            ptr
                        }
                        None => alloc::handle_alloc_error(new_layout),
                    }
                };
                (*new_next_node).has_content = true;

                (*self.node.next.as_ptr()).previous = match NonNull::new(new_next_node) {
                    Some(ptr) => ptr,
                    None => alloc::handle_alloc_error(new_layout),
                };
                self.node.next = match NonNull::new(new_next_node) {
                    Some(ptr) => ptr,
                    None => alloc::handle_alloc_error(new_layout),
                };
            }
        }
    }

    pub fn insert_before(&mut self, element: T) {
        if !self.node.has_content {
            // If the list is empty
            let new_layout = alloc::Layout::new::<T>();
            self.node.content = unsafe {
                let new_ptr = alloc::alloc(new_layout) as *mut T;

                match NonNull::new(new_ptr as *mut T) {
                    Some(ptr) => {
                        *new_ptr = element;
                        ptr
                    }
                    None => alloc::handle_alloc_error(new_layout),
                }
            };
            self.node.has_content = true;
        } else if !self.node.has_previous {
            // If the previous node doesn't exist
            unsafe {
                let new_layout = alloc::Layout::new::<LinkedListNode<T>>();
                let new_previous_node = alloc::alloc(new_layout) as *mut LinkedListNode<T>;

                (*new_previous_node).next = match NonNull::new(self.node) {
                    Some(ptr) => ptr,
                    None => alloc::handle_alloc_error(new_layout),
                };
                (*new_previous_node).has_next = true;
                (*new_previous_node).previous = NonNull::dangling();
                (*new_previous_node).has_previous = false;

                let new_layout = alloc::Layout::new::<T>();

                (*new_previous_node).content = {
                    let new_ptr = alloc::alloc(new_layout) as *mut T;

                    match NonNull::new(new_ptr as *mut T) {
                        Some(ptr) => {
                            *new_ptr = element;
                            ptr
                        }
                        None => alloc::handle_alloc_error(new_layout),
                    }
                };
                (*new_previous_node).has_content = true;

                self.node.previous = match NonNull::new(new_previous_node) {
                    Some(ptr) => ptr,
                    None => alloc::handle_alloc_error(new_layout),
                };
                self.node.has_previous = true;
            }
        } else {
            // General case
            unsafe {
                let new_layout = alloc::Layout::new::<LinkedListNode<T>>();
                let new_previous_node = alloc::alloc(new_layout) as *mut LinkedListNode<T>;

                (*new_previous_node).next = match NonNull::new(self.node) {
                    Some(ptr) => ptr,
                    None => alloc::handle_alloc_error(new_layout),
                };
                (*new_previous_node).has_next = true;
                (*new_previous_node).previous = self.node.previous;
                (*new_previous_node).has_previous = true;

                let new_layout = alloc::Layout::new::<T>();

                (*new_previous_node).content = {
                    let new_ptr = alloc::alloc(new_layout) as *mut T;

                    match NonNull::new(new_ptr as *mut T) {
                        Some(ptr) => {
                            *new_ptr = element;
                            ptr
                        }
                        None => alloc::handle_alloc_error(new_layout),
                    }
                };
                (*new_previous_node).has_content = true;

                (*self.node.previous.as_ptr()).next = match NonNull::new(new_previous_node) {
                    Some(ptr) => ptr,
                    None => alloc::handle_alloc_error(new_layout),
                };
                self.node.previous = match NonNull::new(new_previous_node) {
                    Some(ptr) => ptr,
                    None => alloc::handle_alloc_error(new_layout),
                };
            }
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if self.is_ended {
            return None;
        }

        unsafe {
            if self.node.is_null() {
                None
            } else {
                if (*self.node).has_content {
                    let res = (*self.node).content.as_ref();

                    if (*self.node).has_next {
                        self.node = (*self.node).next.as_ptr();
                    } else {
                        self.is_ended = true;
                    }

                    Some(res)
                } else {
                    None
                }
            }
        }
    }
}
