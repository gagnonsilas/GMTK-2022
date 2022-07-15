extends Node

export var gravity = 0

func _ready():
	randomize()

func randomize():
	gravity = randi() % 6
	
