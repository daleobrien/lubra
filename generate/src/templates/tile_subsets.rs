
pub fn match_side_1 (mask:u32) -> Vec<u32> {
	match mask {
    {% for key, values in side_1 %}
	  {{key}} => vec!{{values}}, {% endfor %}
	   _ => vec!(),
  }
}

pub fn match_side_2 (mask:u32) -> Vec<u32> {
	match mask {
		{% for key, values in side_2 %}
		{{key}} => vec!{{values}}, {% endfor %}
		_ => vec!(),
	}
}

pub fn match_side_3 (mask:u32) -> Vec<u32> {
	match mask {
		{% for key, values in side_3 %}
		{{key}} => vec!{{values}}, {% endfor %}
		_ => vec!(),
	}
}

pub fn match_side_1_2 (mask:u32) -> Vec<u32> {
	match mask {
		{% for key, values in side_1_2 %}
		{{key}} => vec!{{values}}, {% endfor %}
		_ => vec!(),
	}
}

pub fn match_side_2_3 (mask:u32) -> Vec<u32> {
	match mask {
		{% for key, values in side_2_3 %}
		{{key}} => vec!{{values}}, {% endfor %}
		_ => vec!(),
	}
}

pub fn match_side_3_1 (mask:u32) -> Vec<u32> {
	match mask {
		{% for key, values in side_3_1 %}
		{{key}} => vec!{{values}}, {% endfor %}
		_ => vec!(),
	}
}

pub fn match_side_1_2_3 (mask:u32) -> Vec<u32> {
	match mask {
		{% for key, values in side_1_2_3 %}
		{{key}} => vec!{{values}}, {% endfor %}
		_ => vec!(),
	}
}
