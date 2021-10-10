type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
	elem: T,
	next: Link<T>,
}

pub struct LinkedList<T> {
	head: Link<T>,
}

pub struct Iter<'a, T: 'a>(Option<&'a Node<T>>);
pub struct IterMut<'a, T: 'a>(Option<&'a mut Node<T>>);

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None
        }
    }

    pub fn push_front(&mut self, v: T) {
        let rest = self.head.take();
        self.head = Some(Box::new(
            Node {
                elem: v,
                next: rest
            }
        ));
    }

    pub fn iter(&self) -> Iter<T> {
		Iter(self.head.as_ref().map(|node| &**node))
	}

    pub fn iter_mut(&mut self) -> IterMut<T> {
		IterMut(self.head.as_mut().map(|node| &mut **node))
	}
}

impl<'a, T> Iterator for Iter<'a, T> {
	type Item = &'a T;

	fn next(&mut self) -> Option<Self::Item> {
		self.0.take().map(|node| {
			self.0 = node.next.as_ref().map(|node| &**node);
			&node.elem
		})
	}
}

impl<'a, T> Iterator for IterMut<'a, T> {
	type Item = &'a mut T;

	fn next(&mut self) -> Option<Self::Item> {
		self.0.take().map(|node| {
			self.0 = node.next.as_mut().map(|node| &mut **node);
			&mut node.elem
		})
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_linked_list() {
        let mut list: LinkedList<u32> = LinkedList::new();

        list.push_front(0);
        list.push_front(1);
        list.push_front(2);

        for v in list.iter_mut() {
            *v += 10;
        }

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&12));
        assert_eq!(iter.next(), Some(&11));
        assert_eq!(iter.next(), Some(&10));
        assert_eq!(iter.next(), None);
    }
}