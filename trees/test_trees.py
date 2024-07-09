from tree import Node, traverse_tree


def create_tree() -> Node[int]:
    """
    Returns the root/head node
    """
    root = Node[int](11)

    root.left = Node[int](12)
    root.left.left = Node[int](16)
    root.left.right = Node[int](18)

    root.right = Node[int](73)
    root.right.left = Node[int](1)
    root.right.right = Node[int](2)
    return root


def test_pre_order_traversal() -> None:
    correct_path = [11, 12, 16, 18, 73, 1, 2]

    root = create_tree()
    path = traverse_tree(root)

    assert len(path) == len(correct_path)

    for i in range(len(path)):
        assert path[i] == correct_path[i]
