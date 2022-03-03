extends KinematicBody2D


# Called when the node enters the scene tree for the first time.
func _ready():
	ECSController.add_node_to_ecs(self, "Player")
