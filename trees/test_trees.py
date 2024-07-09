from tree import Node, breadth_first_search, traverse_tree


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

def test_breadth_first_search() -> None:
    needles = [12, 18, 73, 11]

    root = create_tree()

    for needle in needles:
        result = breadth_first_search(root, needle) 
        assert result is not None
        assert result == needle

    wrong_needle = 96
    assert breadth_first_search(root, wrong_needle) is None
