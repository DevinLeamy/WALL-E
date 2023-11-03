from wall_e_py import Scene, Transform, Geometry, Light, ray_trace, Camera

scene = Scene()

root = Transform() 
root.scale(2.0, 1.0, 1.0)
root.translate(2.0, 1.0, 1.0)

geometry = Geometry("sphere")
geometry.scale(1.0, 2.0, 1.0)
geometry.translate(1.0, 2.0, 1.0)

light = Light()
light.scale(1.0, 2.0, 1.0)
light.translate(1.0, 2.0, 1.0)

root.add_child(geometry)
root.add_child(light)

scene.set_root(root)

camera = Camera((0, 0, 800), (0, 0, -1), (0, 1, 0), 50)
camera.look_at(1.0, 2.0, 1.0)

ray_trace(scene, camera, 500, 500, "image.png")
