import pytest
from linked_list import LinkedList

class TestLinkedLists:
    def test_creating_and_appending_linked_list(self) -> None:
        new_list = LinkedList()
        new_list.prepend(3)
        new_list.prepend(4)

        assert new_list.head.data == 4
        assert new_list.head.next.data == 3
    
    def test_list_iterator(self) -> None:
        new_list = LinkedList()
        new_list.prepend(3)
        new_list.prepend(1)
        new_list.prepend("Some String")
        new_list.prepend(LinkedList())

        for element in new_list:
            assert element is not None
            assert isinstance(element.data, (int, str, LinkedList))

        with pytest.raises(AttributeError):
            new_list._current

    def test_inserting_at_point(self) -> None:
        ins = LinkedList()

        ins.prepend(1)
        ins.prepend(3)

        ins.insert_at(1, 2)

        checker = 3

        for element in ins:
            assert element.data == checker

            checker -= 1

        assert ins.get(1) == 2
        assert ins.get(0) == 3
        assert ins.get(2) == 1

    def test_removing(self) -> None:
        l = LinkedList()
        l.append(1)
        l.append(2)
        l.append(3)

        two = l.remove(2)

        assert two == 2
        assert l.get(0) == 1
        assert l.get(1) == 3

        assert len(l) == 2
