from typing import Self


class Node[T]:
    data: T
    next: Self | None
    prev: Self | None

    def __init__(
        self, data: T, next: Self | None = None, prev: Self | None = None
    ) -> None:
        self.data = data
        self.next = next
        self.prev = prev


class LinkedList[T]:
    head: Node[T] | None
    tail: Node[T] | None
    length: int

    def __init__(self) -> None:
        self.head = None
        self.tail = None
        self.length = 0

    def append(self, data: T) -> None:
        node = Node(data)
        self.length += 1

        if self.tail is None:
            self.tail = node
            self.head = node
            return

        self.tail.next = node
        node.prev = self.tail
        self.tail = node

    def prepend(self, data: T) -> None:
        node = Node(data)
        self.length += 1

        if self.head is None:
            self.head = node
            self.tail = node
            return

        self.head.prev = node
        node.next = self.head
        self.head = node

    def insert_at(self, index: int, data: T) -> None:
        assert index < self.length, "Index out of range"
        self.length += 1

        new_node = Node(data)
        last_idx = len(self) - 1

        if index == last_idx:
            self.append(new_node)
            return

        elif index == 0:
            self.prepend(new_node)
            return
        
        element = self._get_at(index)

        prev = element.prev

        prev.next = new_node
        new_node.prev = prev

        element.prev = new_node
        new_node.next = element

    def get(self, id: int) -> T:
        assert id < self.length, "id out of range"
    
        element = self._get_at(id)

        return element.data if element else None

    def remove_at(self, id: int) -> T | None:
        element = self._get_at(id)

        if element is None:
            return None
        
        return self._remove_node(element)
 

    def remove(self, item: T) -> T | None:
        element = self.head

        for node in self:
            if node is None:
                break
            elif node.data == item:
                break

            element = element.next

        if element is None:
            return None
        
        return self._remove_node(element)

    def __iter__(self):
        self._current = self.head
        return self

    def __next__(self):
        if self._current is not None:
            current = self._current
            next = current.next

            self._current = next
            return current
        else:
            delattr(self, "_current")
            raise StopIteration

    def __len__(self) -> int:
        return self.length

    def _get_at(self, id: int) -> Node[T]:
        if id == 0:
            return self.head
        elif id == self.length - 1:
            return self.tail

        element = self.head.next
        for _ in range(1, id):
            element = element.next

        return element

    def _remove_node(self, element: Node[T]) -> T | None:
        self.length -= 1

        if self.length == 0:
            self.head = self.tail = None

            return element.data
        
        self._break_node_connections(element)

        return element.data 

    def _break_node_connections(self, element: Node[T]) -> None:
        if element.prev is not None:
            element.prev.next = element.next

        if element.next is not None:
            element.next.prev = element.prev

        if element == self.head:
            self.head = element.next

        if element == self.tail:
            self.tail = element.prev

        element.prev = element.next = None
