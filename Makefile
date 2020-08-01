run:
	#make the resources
	cd src && glib-compile-resources app.xml
	cargo run --release

debug:
	#make the resources
	cd src && glib-compile-resources app.xml
	cargo run
