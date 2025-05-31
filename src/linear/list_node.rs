/// A node for linked list data structure
///
/// # Examples
/// ```
/// use data_structure::linear::list_node::ListNode;
/// let mut head = ListNode::<i32>::new();
/// head.push(1);
/// head.push(2);
/// ```
pub struct ListNode<T> {
    /// The data stored in this node, None for sentinel node
    pub data: Option<T>,
    /// Pointer to the next node
    pub next: Option<Box<ListNode<T>>>,
}

impl<T> ListNode<T> {
    /// Creates a new sentinel node (with no data)
    ///
    /// # Examples
    /// ```
    /// use data_structure::linear::list_node::ListNode;
    /// let sentinel = ListNode::<i32>::new();
    /// assert!(sentinel.data.is_none());
    /// ```
    pub fn new() -> Self {
        ListNode {
            data: None,
            next: None,
        }
    }

    /// Gets a reference to the node at the given index (0-based)
    /// Returns None if index is out of bounds
    ///
    /// # Examples
    /// ```
    /// use data_structure::linear::list_node::ListNode;
    /// let mut head = ListNode::new();
    /// head.push(42);
    /// assert_eq!(head.get(1).unwrap().data, Some(42));
    /// ```
    pub fn get(&self, index: usize) -> Option<&Self> {
        let mut current = self;
        for _ in 0..index {
            current = current.next.as_ref()?;
        }
        Some(current)
    }

    /// Gets a mutable reference to the node at the given index (0-based)
    /// Returns None if index is out of bounds
    ///
    /// # Examples
    /// ```
    /// use data_structure::linear::list_node::ListNode;
    /// let mut head = ListNode::new();
    /// head.push(42);
    /// *head.get_mut(1).unwrap().data.as_mut().unwrap() = 24;
    /// ```
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Self> {
        let mut current = self;
        let mut count = 0;
        // 边界检查：当 index 为 0 时直接返回当前节点
        if index == 0 {
            return Some(current);
        }
        while count < index - 1 {
            // 使用 as_mut()? 自动处理 None 情况
            current = current.next.as_mut()?;
            count += 1;
        }
        // 最后一次移动需要特别检查
        current.next.as_mut().map(|node| &mut **node)
    }

    /// Removes the last node in the list
    ///
    /// # Panics
    /// Panics if called on an empty list (only sentinel node exists)
    ///
    /// # Examples
    /// ```
    /// use data_structure::linear::list_node::ListNode;
    /// let mut head = ListNode::new();
    /// head.push(1);
    /// head.push(2);
    /// head.pop_tail();
    /// assert!(head.get(1).unwrap().next.is_none());
    /// ```
    pub fn pop_tail(&mut self) {
        let mut current = self;
        while current.next.is_some() {
            current = current.next.as_mut().unwrap();
        }
        current.next = None;
    }

    /// Appends a new node with given data to the end of the list
    ///
    /// # Examples
    /// ```
    /// use data_structure::linear::list_node::ListNode;
    /// let mut head = ListNode::new();
    /// head.push(42);
    /// assert!(head.next.is_some());
    /// ```
    pub fn push(&mut self, data: T) {
        let mut tail = self;
        while tail.next.is_some() {
            tail = tail.next.as_mut().unwrap();
        }
        tail.next = Some(Box::new(ListNode {
            data: Some(data),
            next: None,
        }));
    }
}

#[cfg(test)]
mod tests {
    use super::ListNode;
    #[test]
    fn test_sentinel_head() {
        let mut head: Box<ListNode<i32>> = Box::new(ListNode::new());
        for i in 1..6 {
            head.push(i);
        }
        assert!(head.data.is_none());
        assert_eq!(head.next.unwrap().data, Some(1))
    }
    #[test]
    fn test_get_mut() {
        let mut head: Box<ListNode<i32>> = Box::new(ListNode::new());
        for i in 1..6 {
            head.push(i);
        }
        let zero = head.get(0).unwrap();
        assert!(zero.data.is_none());
        let value = head.get_mut(1).unwrap();
        assert_eq!(value.data, Some(1));
    }

}
