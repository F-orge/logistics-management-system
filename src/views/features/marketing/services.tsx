import { Laptop } from "lucide-solid";
import { For, type Component } from "solid-js";
import { Card, CardContent, CardHeader, CardTitle } from "~/components/ui/card";

const COUNTRIES = [
	"Philippines",
	"United States",
	"Canada",
	"United Kingdom",
	"Germany",
	"France",
	"Italy",
	"Spain",
	"Australia",
	"Japan",
	"China",
	"India",
	"Brazil",
	"Mexico",
	"Russia",
	"South Korea",
	"Netherlands",
	"Switzerland",
	"Sweden",
	"Norway",
	"Denmark",
	"Finland",
	"Belgium",
	"Austria",
	"Ireland",
];

const SERVICES = [
	{
		icon: <Laptop />,
		title: "International and Domestic Freight Forwarding",
		description:
			"Providing comprehensive sea, air, and land freight forwarding services for both international and domestic shipments.",
	},
	{
		icon: <Laptop />,
		title: "Relocation Services",
		description:
			"Offering seamless international and domestic relocation services to ensure a smooth transition.",
	},
	{
		icon: <Laptop />,
		title: "Import / Export Services",
		description:
			"Facilitating efficient import and export services to streamline your global trade operations.",
	},
	{
		icon: <Laptop />,
		title: "Customs Clearance",
		description:
			"Ensuring hassle-free customs clearance for your shipments with our expert services.",
	},
	{
		icon: <Laptop />,
		title: "Trucking",
		description:
			"Providing reliable and timely trucking services for your transportation needs.",
	},
	{
		icon: <Laptop />,
		title: "Door to Door Delivery Nationwide",
		description:
			"Offering nationwide door to door delivery services to ensure your packages reach their destination safely.",
	},
];

const ServicesSection: Component<{}> = (props) => {
	return (
		<section class="py-12 rounded-md px-4 w-full flex flex-col gap-20 overflow-x-hidden relative">
			<div class="text-center">
				<span class="text-primary">Our services</span>
				<h2 class="heading-2">Ship to over 20+ countries</h2>
			</div>
			<div class="w-full inline-flex flex-nowrap">
				<For each={[...new Array(8)]}>
					{(_, index) => (
						<ul class="flex items-center justify-center md:justify-start [&_li]:mx-8 [&_img]:max-w-none animate-infinite-scroll">
							<For each={COUNTRIES}>
								{(country) => <li class="small">{country}</li>}
							</For>
						</ul>
					)}
				</For>
			</div>
			<div class="grid phone:grid-cols-1 desktop:grid-cols-2 gap-5">
				<For each={SERVICES}>
					{(feature) => (
						<Card class="shadow-lg intersect:animate-fade-up animate-delay-300 intersect-once">
							<CardHeader class="flex flex-row items-center gap-2.5">
								{feature.icon}
								<CardTitle>{feature.title}</CardTitle>
							</CardHeader>
							<CardContent>{feature.description}</CardContent>
						</Card>
					)}
				</For>
			</div>
		</section>
	);
};

export default ServicesSection;
