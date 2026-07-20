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


def width_aware_blocks(
    first_positional_parameter_with_a_long_name,
    second_positional_parameter_with_a_long_name,
    /,
    *,
    first_keyword_parameter_with_a_long_name,
    second_keyword_parameter_with_a_long_name,
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


render(
    [
        1,
        2,
        3,
    ]
)


configure(
    {
        "enabled": True,
        "options": ["a", "b"],
    }
)
