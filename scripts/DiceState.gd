extends Node

class_name DiceState

export var gravity = 0

func get_gravity():
	return gravity

func randomize():
	gravity = randi() % 6
	
