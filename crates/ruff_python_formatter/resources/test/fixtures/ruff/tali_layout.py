def keyword_groups(self, *,
    first,
    second,
):
    pass


def positional_groups(
    a, b, /,
    c, d,
):
    pass


def semantic_groups(
    a,
    b,
    /,
    c,
    d,
    *,
    e,
    f,
):
    pass


def adjacent_markers(
    a,
    b,
    /,
    *,
    c,
    d,
):
    pass


matrix = [
    [1,   20, 300],
    [400, 5,  6],
]


normalized_matrix = [
    [1.E2,  3],
    [40.0, 500],
]


decimal_matrix = [
    [1.2,   30.45],
    [100, 4.0],
]


unaligned_matrix = [
    [1, 20],
    [300, 4],
]


tensor = [
    [
        [1,  20],
        [300, 4],
    ],
    [
        [50, 6],
        [7,  800],
    ],
]


ragged = [
    [1,   2],
    [3, 4, 5],
]


render([
    1,
    2,
    3,
])


configure({
    "enabled": True,
    "options": ["a", "b"],
})
