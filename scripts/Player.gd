extends KinematicBody2D


export var speed = 200
export var gravity = 10
export var jump = 400

export var vel = Vector2.ZERO

# Declare member variables here. Examples:
# var a = 2
# var b = "text"

func _physics_process(delta):
	vel.x = 0
	
	if Input.is_action_pressed("left"):
		vel.x += speed
	if Input.is_action_pressed("right"):
		vel.x -= speed
	
	if Input.is_action_pressed("up") && is_on_floor():
		vel.y = -jump
	
	vel.y += gravity
	
	vel = move_and_slide(vel, Vector2.UP)
		

# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass
