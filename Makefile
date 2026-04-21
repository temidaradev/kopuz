serve:
	dx serve

tailwind:
	npx @tailwindcss/cli -i ./tailwind.css -o ./kopuz/assets/tailwind.css --content './kopuz/**/*.rs,./components/**/*.rs,./pages/**/*.rs,./hooks/**/*.rs,./player/**/*.rs,./reader/**/*.rs'

build: tailwind
	dx build --package kopuz --release
	@echo ""
	@echo "Build complete!"

run-release:
	cd target/dx/kopuz/release/linux/app && ./kopuz

flatpak:
	@chmod +x build-flatpak.sh
	./build-flatpak.sh

flatpak-install: flatpak

flatpak-run:
	flatpak run com.temidaradev.kopuz

clean:
	cargo clean
	rm -rf target/dx dist build-dir .flatpak-builder

