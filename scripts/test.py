from wall_e_py import Scene, Transform, Geometry, Light, ray_trace, Camera, Material

red = Material((1.0, 0.0, 0.0), (0.1, 0.1, 0.1), 1.0)
green = Material((0.4, 1.0, 0.4), (0.5, 0.5, 0.5), 1.0)
blue = Material((0.1, 0.1, 1.0), (0.5, 0.5, 0.5), 0.8)
scene = Scene()

root = Transform()
root.scale(2.0, 1.0, 1.0)
root.translate(0.0, 0.0, 0.0)

# geometry = Geometry("sphere")
geometry = Geometry("cube")
geometry.scale(1.0, 1.0, 1.0)
geometry.translate(0.0, 0.0, 0.0)
geometry.set_material(green)

light = Light()
light.scale(1.0, 1.0, 1.0)
light.translate(2.0, 4.0, -8.0)

root.add_child(geometry)
root.add_child(light)

camera = Camera((0, 0, 5.0), (0, 0, -1), (0, 1, 0), 50)
camera.set_position(2.0, 2.0, -5.0)
camera.look_at(0.0, 0.0, 0.0)

scene.set_root(root)
# ray_trace(scene, camera, 20, 20, "image.png")
# ray_trace(scene, camera, 90, 90, "image.png")
print("trace")
ray_trace(scene, camera, 300, 300, "image.png")
# ray_trace(scene, camera, 1000, 800, "image.png")