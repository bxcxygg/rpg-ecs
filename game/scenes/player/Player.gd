extends KinematicBody2D


# Called when the node enters the scene tree for the first time.
func _ready():
	ECSController.add_node_to_ecs(self, "Player")
	
func attack_animation_finished():
	pass
	#ECSController.add_signal_to_ecs("attack_animation_finished",[])
	
func roll_animation_finished():
	pass
	#ECSController.add_signal_to_ecs("roll_animation_finished",[])
