#[derive(Clone)]
pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

#[derive(Clone)]
pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList::<T>{ head: Option::None }
    }

    pub fn len(&self) -> usize {
        let mut len = 0;
        let mut np = &(self.head);
        while (*np).is_some() {
            len += 1;
            np = &(np.as_ref().unwrap().next);
        }
        len
    }

    pub fn push(&mut self, _element: T) {
        self.head = Some(Box::new(Node {
            data: _element,
            next: self.head.take()
        }))
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }
        let mut n = self.head.take().unwrap();
        self.head = n.next.take();
        Some(n.data)
    }

    pub fn peek(&self) -> Option<&T> {
        match self.head {
            None => None,
            _ => Some(&self.head.as_ref().unwrap().data),
        }
    }

    fn loop_into(ref _node: &Option<Box<Node<T>>>) -> Option<Node<T>> {
        unimplemented!()
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        let mut list = Self::new();
        let mut np = &(self.head);
        while np.is_some() {
            list.push(np.as_ref().unwrap().data.clone());
            np = &(np.as_ref().unwrap().next);
        }
        list
    }
}

impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(_item: &[T]) -> Self {
        let mut list = Self::new();
        for it in _item.iter() {
            list.push(it.clone());
        }
        list
    }
}

impl<T: Clone> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut vec : Vec<T> = Vec::new();
        let mut np = &(self.head);
        while let Some(n) = np {
            vec.insert(0, np.as_ref().unwrap().data.clone());
            np = &(np.as_ref().unwrap().next);
        }
        vec
    }
}
