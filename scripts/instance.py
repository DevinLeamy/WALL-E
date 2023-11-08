from wall_e_py import Mesh, Scene, Transform, Geometry, Light, ray_trace, Camera, Material
from shared import copy_and_archive_image

stone = Material((0.8, 0.7, 0.7), (0.0, 0.0, 0.0), 0)
grass = Material((0.1, 0.7, 0.1), (0.0, 0.0, 0.0), 0)
scene = Scene()

arc = Transform()
arc.translate(0, 0, -10)

p1 = Geometry("cube")
p1.set_material(stone)
p1.scale(0.8, 4, 0.8)
p1.translate(-2.4, 0, -0.4)
arc.add_child(p1)

p2 = Geometry("cube")
p2.set_material(stone)
p2.scale(0.8, 4, 0.8)
p2.translate(1.6, 0, -0.4)
arc.add_child(p2)

s = Geometry("sphere")
s.set_material(stone)
s.scale(4, 0.6, 0.6)
s.translate(0, 4, 0)
arc.add_child(s)

n_scene = Transform()
n_scene.rotate('X', 23)

plane = Mesh('plane.obj')
plane.set_material(grass)
plane.scale(30, 30, 30)
n_scene.add_child(plane)

sphere = Geometry('sphere')
sphere.scale(2.5, 2.5, 2.0)
sphere.set_material(stone)
n_scene.add_child(sphere)

for i in range(6):
    n_arc = Transform()
    n_arc.rotate('Y', (i-1) * 60)
    n_scene.add_child(n_arc)
    n_arc.add_child(arc)

n_scene.add_child(arc)


light = Light()
light.translate(0.8, 0.8, 0.8)

n_scene.add_child(light)
camera = Camera((0, 2, 30), (0, 0, -1), (0, 1, 0), 50)
scene.set_root(n_scene)

copy_and_archive_image()

ray_trace(scene, camera, 256, 256, "image.png")
