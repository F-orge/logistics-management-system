import type { Component } from "solid-js";
import { Button } from "~/components/ui/button";

const HeroSection: Component<{}> = (props) => {
	return (
		<section class="py-24 flex flex-col justify-center items-center gap-5">
			<h1 class="heading-1 text-primary text-center">Going Beyond Logistics</h1>
			<p class="paragraph desktop:w-1/2 text-center">
				ETMAR exports premium Philippine products worldwide.
			</p>
			<ul class="flex flex-row gap-5">
				<li>
					<Button variant="default">Request a quote</Button>
				</li>
			</ul>
		</section>
	);
};

export default HeroSection;
