extends AnimatedSprite

func _on_animation_finished():
	ECSController.add_signal_to_ecs("Effect/_on_animation_finished", [self])
