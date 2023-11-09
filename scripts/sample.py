from wall_e_py import Mesh, Scene, Transform, Light, Camera, Material, Geometry, ray_trace
from shared import copy_and_archive_image
import random

wood = Material((0.8, 0.7, 0.7), (0.0, 0.0, 0.0), 0)
leaves = Material((0.8, 0.3, 0.3), (0.0, 0.0, 0.0), 0)
leaves2 = Material((0.8, 0.5, 0.5), (0.0, 0.0, 0.0), 0)
grass = Material((0.1, 0.7, 0.1), (0.0, 0.0, 0.0), 0)
tree_leaves = Material((0.4, 0.7, 0.4), (0.0, 0.0, 0.0), 0)
car_side = Material((0.84, 0.9, 0.53), (0.3, 0.3, 0.3), 20)
car_wheel = Material((0.84, 0.6, 0.53), (0.3, 0.3, 0.3), 20)

scene_root = Transform()

plane = Mesh('plane.obj')
plane.set_material(grass)
plane.scale(90, 90, 90)
scene_root.add_child(plane)


def create_car():
    tree = Transform()
    h = 6.0
    parts = [
        # Trunk
        ("cube", (0, 0, 0), (1.5, h, 1.5), wood),
        # Leaves
        ("sphere", (1.0, h + 0.3, 1), (2.0, 2.0, 2.0), tree_leaves),
        ("sphere", (1.0, h + 1.7, 1), (1.8, 1.8, 1.8), tree_leaves),
        ("sphere", (1.4, h + 2.4, 1), (1.6, 1.6, 1.6), tree_leaves),
    ]
    for geometry, position, scale, material in parts:
        part = Geometry(geometry)
        part.set_material(material)
        part.scale(scale[0], scale[1], scale[2])
        part.translate(position[0], position[1], position[2])
        tree.add_child(part)

    return tree


def create_tree():
    tree = Transform()
    h = 6.0
    parts = [
        # Trunk
        ("cube", (0, 0, 0), (1.5, h, 1.5), wood),
        # Leaves
        ("sphere", (1.0, h + 0.3, 1), (2.0, 2.0, 2.0), tree_leaves),
        ("sphere", (1.0, h + 1.7, 1), (1.8, 1.8, 1.8), tree_leaves),
        ("sphere", (1.4, h + 2.4, 1), (1.6, 1.6, 1.6), tree_leaves),
    ]
    for geometry, position, scale, material in parts:
        part = Geometry(geometry)
        part.set_material(material)
        part.scale(scale[0], scale[1], scale[2])
        part.translate(position[0], position[1], position[2])
        tree.add_child(part)

    return tree


def create_leaves():
    n_leaves = Transform()
    h = -0.5
    parts = [
        # Leaves
        ("sphere", (-0.2, h, -0.2), (1.0, 1.0, 1.0), leaves),
        ("sphere", (0, h, 0), (1.8, 1.8, 1.8), leaves),
        ("sphere", (-0.4, h, 0.5), (1.6, 1.6, 1.6), leaves2),
    ]
    for geometry, position, scale, material in parts:
        part = Geometry(geometry)
        part.set_material(material)
        part.scale(scale[0], scale[1], scale[2])
        part.translate(position[0], position[1], position[2])
        n_leaves.add_child(part)

    return n_leaves


x = [-30, 30]
z = [20, -40]

random.seed(41)

tree_positions = []
n_trees = 10
for i in range(n_trees):
    x_pos = random.uniform(x[0], x[1])
    z_pos = random.uniform(z[0], z[1])
    tree_positions.append(((x_pos, 1.3, z_pos), 0))

random.seed(47)
leaves_positions = []
n_leaves = 6
for i in range(n_leaves):
    x_pos = random.uniform(x[0], x[1])
    z_pos = random.uniform(z[0], z[1])
    leaves_positions.append(((x_pos, 1.3, z_pos), random.uniform(0, 180)))

for position, rotation in tree_positions:
    tree_instance = Transform()
    tree_instance.add_child(create_tree())
    tree_instance.scale(1.4, 1.4, 1.4)
    tree_instance.rotate('Y', rotation)
    tree_instance.translate(position[0], position[1], position[2])
    scene_root.add_child(tree_instance)

for position, rotation in leaves_positions:
    leaves_instance = Transform()
    leaves_instance.add_child(create_leaves())
    leaves_instance.scale(1.4, 1.4, 1.4)
    leaves_instance.rotate('Y', rotation)
    leaves_instance.translate(position[0], position[1], position[2])
    scene_root.add_child(leaves_instance)

light_source = Light((0.8, 0.8, 0.8), (1, 0, 0))
light_source.translate(50, 202, -130)

light_source2 = Light((0.4, 0.4, 0.4), (1, 0, 0))
light_source2.translate(-50, 202, 130)
scene_root.add_child(light_source)
scene_root.add_child(light_source2)

scene = Scene()
scene.set_root(scene_root)
scene.set_ambient(0.2, 0.2, 0.2)

camera = Camera((0, 30, 30), (0, 0, -1), (0, 1, 0), 80)
camera.look_at(0, 0, 0)
# ray_trace(scene, camera, 50, 50, "image.png")
ray_trace(scene, camera, 100, 100, "image.png")

copy_and_archive_image()
