#[cfg(test)]
mod tests {
    use crate::list::ArrayList;
    //    use super::*;

    #[test]
    fn test1() {
        let mut arr:ArrayList<char>  = ArrayList::new();
        arr.insert(1, 'a').expect("TODO: panic message");
        arr.insert(2, 'b').expect("TODO: panic message");
        arr.insert(3, 'c').expect("TODO: panic message");
        // assert_eq!(arr.get_element(1), Ok('a'));
        // assert_eq!(arr.length(), 3);
        // let index=arr.locate_index('c').unwrap();
        // assert_eq!(index, 3);
        //let result=arr.get_element(4).unwrap();
        //println!("{:?}",result);
        //assert_eq!(result,'c');
    }
}
pub mod list {
    /// 一个基于数组的简单列表实现。
    ///
    /// 这个结构体提供了一个基本的基于数组的列表，具有固定的容量。
    /// 它支持插入、删除和遍历等操作。
    pub struct ArrayList<T> {
        /// 存储元素的底层数组。
        pub element: [Option<T>; 100],
        /// 列表中当前元素的数量。
        pub length: usize,
    }

    impl<T> ArrayList<T>
    where
        T: Copy + PartialEq + std::fmt::Debug,
    {
        /// 创建一个新的空 `ArrayList`。
        ///
        /// # 示例
        ///
        /// ```
        /// use data_structure::list::ArrayList;
        /// let list: ArrayList<i32> = ArrayList::new();
        /// assert_eq!(list.length(), 0);
        /// ```
        pub fn new() -> Self {
            Self {
                element: [None; 100],
                length: 0,
            }
        }

        /// 返回指定位置的元素。
        ///
        /// # 参数
        ///
        /// * `index` - 要检索的元素的位置（从1开始的索引）。
        ///
        /// # 返回值
        ///
        /// 如果成功，返回包含元素的 `Result`，否则返回错误信息。
        ///
        /// # 错误
        ///
        /// 如果索引超出范围或元素不存在，则返回错误。
        pub fn get_element(&self, index: usize) -> Result<T, &'static str> {
            if index == 0 {
                return Err("数组越界");
            }
            let idx = index - 1;
            if idx >= self.length {
                return Err("数组越界");
            };
            match self.element[idx] {
                Some(value) => Ok(value),
                None => Err("元素不存在!"),
            }
        }

        /// 在指定位置插入元素。
        ///
        /// # 参数
        ///
        /// * `position` - 插入元素的位置（从1开始的索引）。
        /// * `element` - 要插入的元素。
        ///
        /// # 返回值
        ///
        /// 表示成功或错误信息的 `Result`。
        ///
        /// # 错误
        ///
        /// 如果位置无效或数组已满，则返回错误。
        pub fn insert(&mut self, position: usize, element: T) -> Result<(), &'static str> {
            if position == 0 || position > self.length + 1 {
                return Err("数组越界");
            };
            let index = position - 1;
            if self.length >= 100 {
                return Err("数组已满");
            };
            for i in (index..self.length).rev() {
                self.element[i + 1] = self.element[i];
            };
            self.element[index] = Some(element);
            self.length += 1;
            Ok(())
        }

        /// 查找指定元素的第一个出现位置。
        ///
        /// # 参数
        ///
        /// * `element` - 要查找的元素。
        ///
        /// # 返回值
        ///
        /// 如果找到，返回包含索引的 `Result`，否则返回错误信息。
        ///
        /// # 错误
        ///
        /// 如果未找到元素，则返回错误。
        pub fn locate_index(&self, element: T) -> Result<usize, &'static str> {
            for i in 0..self.length {
                if let Some(value) = self.element[i] {
                    if value == element {
                        return Ok(i + 1);
                    }
                }
            }
            Err("查找失败")
        }

        /// 删除指定位置的元素。
        ///
        /// # 参数
        ///
        /// * `index` - 要删除的元素的位置（从1开始的索引）。
        ///
        /// # 返回值
        ///
        /// 表示成功或错误信息的 `Result`。
        ///
        /// # 错误
        ///
        /// 如果索引超出范围，则返回错误。
        pub fn delete(&mut self, index: usize) -> Result<(), &'static str> {
            if index == 0 || index > self.length {
                return Err("数组操作越界!");
            };
            for index in index..self.length {
                self.element[index - 1] = self.element[index];
            }
            Ok(())
        }

        /// 清除列表中的所有元素。
        pub fn clear(&mut self) {
            for index in 0..self.length {
                self.element[index] = None;
            };
            self.length = 0;
        }

        /// 返回列表中当前元素的数量。
        pub fn length(&self) -> usize {
            self.length
        }

        /// 检查列表是否为空。
        ///
        /// # 返回值
        ///
        /// 如果列表不为空，返回 `true`，否则返回 `false`。
        pub fn empty(&self) -> bool {
            self.length != 0
        }

        /// 返回指定元素的前驱元素。
        ///
        /// # 参数
        ///
        /// * `cur_e` - 当前元素。
        ///
        /// # 返回值
        ///
        /// 如果找到，返回包含前驱元素的 `Result`，否则返回错误信息。
        ///
        /// # 错误
        ///
        /// 如果列表为空、当前元素为第一个元素或未找到元素，则返回错误。
        pub fn prior_element(&self, cur_e: T) -> Result<T, &'static str> {
            if self.empty() {
                return Err("数组为空");
            };
            if self.element[0] == Some(cur_e) {
                return Err("当前元素为第一个元素,没有前驱");
            };
            for index in 1..self.length {
                if self.element[index] == Some(cur_e) {
                    return Ok(self.element[index - 1].unwrap());
                };
            };
            Err("元素不存在!")
        }

        /// 返回指定元素的后继元素。
        ///
        /// # 参数
        ///
        /// * `cur_e` - 当前元素。
        ///
        /// # 返回值
        ///
        /// 如果找到，返回包含后继元素的 `Result`，否则返回错误信息。
        ///
        /// # 错误
        ///
        /// 如果列表为空、当前元素为最后一个元素或未找到元素，则返回错误。
        pub fn next_element(&self, cur_e: T) -> Result<T, &'static str> {
            if self.empty() {
                return Err("数组为空");
            }
            if self.element[self.length - 1] == Some(cur_e) {
                return Err("当前元素为最后一个元素,没有后继");
            }
            for index in 0..self.length - 1 {
                if self.element[index] == Some(cur_e) {
                    return Ok(self.element[index + 1].unwrap());
                }
            }
            Err("元素不存在!")
        }

        /// 遍历列表并打印所有元素。
        pub fn traverse(&self) {
            for index in 0..self.length {
                println!("{:?}", self.element[index].unwrap());
            }
        }
    }
}
