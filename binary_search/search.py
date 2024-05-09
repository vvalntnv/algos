def search_binary(array: list[int], target: int) -> bool:
    """
    Search for an element in an array and return
    True if the element is part of an array
    """
    start = 0
    end = len(array) - 1
    middle = end // 2

    while start <= end:
        if array[middle] == target:
            return True
        elif array[middle] > target:
            end = middle - 1
        else:
            start = middle + 1
       
        middle = (start + end) // 2

    return False


def search_binary_recursive(array: list[int], target: int, middle = 0) -> bool:
    """
    Search for an element in an array and return
    True if the element is part of an array
    """

if __name__ == "__main__":
    arr = [1, 3, 5, 6, 12, 14, 16, 19, 20]

    assert search_binary(arr, 4) == False 
    assert search_binary(arr, 20) == True
    assert search_binary(arr, 16) == True
    assert search_binary(arr, 13) == False
    assert search_binary(arr, 6) == True
    assert search_binary(arr, 1) == True

    print("All tests passed!")
