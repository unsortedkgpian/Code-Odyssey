const engine = {
	working:true
};

const mustang = {
	name: "Mustang",
	engine:engine
};

const camelo = {
	name : "Camelo",
	engine: engine
};

function checkEngine(car) {
	if (car.name == "Mustang") {
		car.engine.working = false;
	}
};


