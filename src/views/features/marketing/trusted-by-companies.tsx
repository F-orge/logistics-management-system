import { For, type Component } from "solid-js";
import {
	Card,
	CardContent,
	CardFooter,
	CardHeader,
} from "~/components/ui/card";

const TRUSTED_BY_COMPANIES = [
	{
		name: "Transistor",
		logo: "https://tailwindui.com/plus/img/logos/transistor-logo-gray-900.svg",
	},
	{
		name: "Reform",
		logo: "https://tailwindui.com/plus/img/logos/reform-logo-gray-900.svg",
	},
	{
		name: "Tuple",
		logo: "https://tailwindui.com/plus/img/logos/tuple-logo-gray-900.svg",
	},
	{
		name: "SavvyCal",
		logo: "https://tailwindui.com/plus/img/logos/savvycal-logo-gray-900.svg",
	},
	{
		name: "statamic",
		logo: "https://tailwindui.com/plus/img/logos/statamic-logo-gray-900.svg",
	},
];

const TrustedByCompaniesSection: Component<{}> = (props) => {
	return (
		<section class="py-12 rounded-md px-4 w-full flex flex-col gap-5 ">
			<span class="heading-2 text-center desktop:text-start">
				Trusted by the world’s most innovative companies
			</span>
			<div class="grid grid-cols-1 desktop:grid-cols-5 gap-5 p-4">
				<For each={TRUSTED_BY_COMPANIES}>
					{(company) => (
						<div class="flex items-center justify-center">
							<img class="h-16" src={company.logo} alt={company.name} />
						</div>
					)}
				</For>
			</div>
		</section>
	);
};

export default TrustedByCompaniesSection;
