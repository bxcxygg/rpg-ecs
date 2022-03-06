extends Node2D


func _ready():
	ECSController.add_node_to_ecs(self,"Grass")

func _on_area_entered(area):
	ECSController.add_signal_to_ecs("grass/_on_area_entered",[self, area])
