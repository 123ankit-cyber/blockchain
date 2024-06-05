struct TreeNode{
    val:i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,

}
impl TreeNode{
    fn new(val: i32)->Self{
        TreeNode{
            val,
            left:None,
            right:None,
        }
    }
}

fn max_depth(root:& Option<Box<TreeNode)->i32{
    match root{
        Some(node)=> 1+i32::max(max_depth(&node.left),max_depth(&node,right)),
        None=>0,
    }
}
fn main()
{
    let mut root = s(Box::new(TreeNode::new(1)));
    if let Some(ref mut node)=root{
        node.left=Some(Box::new(TreeNode::new(2)));
        node.right=Some(Box::new(TreeNode::new(3)));
        if let Some(ref mut left_node)=node.left{
            left_node.left=Some(Box:: new(TreeNode::new(4)));
            left_node.right=Some(Box:: new(TreeNode::new(5)));
        }

    }
    println!("the maximum depth of the tree is {}",max_depth(&root));
}