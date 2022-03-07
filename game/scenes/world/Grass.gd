extends Node2D


func _ready():
	ECSController.add_node_to_ecs(self,"Grass")
