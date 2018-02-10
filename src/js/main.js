function toggle_uncles(node) {
	node = node.parentElement.nextElementSibling;
	while(node != null) {
		new_classes = node.className.replace(/(?:^|\s)hidden(?!\S)/, '');
		
		if(new_classes == node.className) {
			node.className += " hidden";
		} else {
			node.className = new_classes;
		}
		
		node = node.nextElementSibling;
	}
}

function random_string(n) {
	var alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
	return Array(n).join().split(',').map(function () {return alphabet.charAt(Math.floor(Math.random() * alphabet.length)); }).join('');
}

function fill_value_from_cookie(query, cookie) {
	nodes = document.querySelectorAll(query);

	match = document.cookie.match(new RegExp("(?:((^|)" + cookie + "=)).*?(?=(;|$))"));
	if(match == null) {
		val = random_string(10);
		document.cookie = (cookie + "=" + val + "; path=/");
	} else {
		val = match[0].split('=')[1];
	}

	for (i = 0; i < nodes.length; i++) {
		nodes[i].value = val;
	}
}

function save_password_cookie(id) {
	document.cookie = "password=" + document.getElementById(id).value + "; path=/";
}

function toggle_image_display(img) {
	if(img.className != "") {
		img.className = "";
	} else {
		img.className = "full-img";
	}
}

function attach_image_click_callback() {
	images = document.querySelectorAll("img");

	for(i = 0; i < images.length; i++) {
		images[i].onclick = function() { toggle_image_display(this); };
		//The 'this' is actually going to be images[i] thanks to javascript's magical scope rules.
	}
}
