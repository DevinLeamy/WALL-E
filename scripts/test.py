from wall_e_py import Scene, Node, Geometry

scene = Scene()

node = Node("transformation")
node.scale_nonuniform((2.0, 1.0, 1.0))

geometry = Geometry("sphere")
geometry.scale_nonuniform((1.0, 2.0, 1.0))

scene.add_child(node)

print("Done")
