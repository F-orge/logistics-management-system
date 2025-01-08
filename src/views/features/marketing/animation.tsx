export function executeAnimations() {
	const observer = new IntersectionObserver((entries) => {
		for (const entry of entries) {
			if (entry.isIntersecting) {
				entry.target.classList.toggle("animate-fade-up");
			}
		}
	});
	const element = document.body.querySelectorAll(".animate-fade-up");
	if (element) {
		for (const el of Array.from(element)) {
			observer.observe(el);
		}
	}
}
