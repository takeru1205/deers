import deers


def test_graph():
    g = deers.Graph()
    g.add_node(1)

    print(g.str())


def test_add_node():
    g = deers.Graph()
    for i in range(10):
        g.add_node(i)

    print(g.str())


def test_name():
    name = "test"
    named_g = deers.Graph(name=name)
    named_g.add_node(1)
    assert name == named_g.name


def test_set_name():
    name = "test"
    named_g = deers.Graph()
    named_g.add_node(1)
    named_g.name = name
    assert name == named_g.name


def test_update_name():
    name = "test"
    named_g = deers.Graph(name=name)
    named_g.add_node(1)
    assert name == named_g.name
    new_name = "test_update"
    named_g.add_node(2, name=new_name)
    assert new_name == named_g.name
