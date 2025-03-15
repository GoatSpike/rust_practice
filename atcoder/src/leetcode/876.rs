Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

// Solution 構造体を定義し、その中に関数を実装
pub struct Solution;
impl Solution {
  // 単一リンクリストの中央ノードを見つける関数
  pub fn middle_node(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {


  // ノードの参照を格納するベクター
  let mut vecters = Vec::new();


  // リンクリストをトラバースしてノードをベクターに収集
  while let Some(node) = head {
    vecters.push(node.clone());
    head = node.next;
  }
  // 中間ノードを返す
  vecters.get(vecters.len() / 2).cloned()
  }
}
fn main() {
  // 使用例がここに来る (省略)
}
