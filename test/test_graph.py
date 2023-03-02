import deers


def test_graph():
    g = deers.Graph()
    g.add_node(1)

    print(g.str())
