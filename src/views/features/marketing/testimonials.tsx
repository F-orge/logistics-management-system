import { For, type Component } from "solid-js";
import { Avatar, AvatarImage } from "~/components/ui/avatar";
import {
	Card,
	CardContent,
	CardDescription,
	CardFooter,
	CardHeader,
	CardTitle,
} from "~/components/ui/card";

const TESTIMONAILS = [
	{
		avatar:
			"https://images.unsplash.com/photo-1550525811-e5869dd03032?ixlib=rb-=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=1024&h=1024&q=80",
		name: "Brenna Goyette",
		username: "@brennagoyette",
		content:
			'"ETMAR Logistics has been an invaluable partner in our business. They have helped us streamline our logistics operations and have always been reliable in delivering our products on time."',
	},
	{
		avatar:
			"https://images.unsplash.com/photo-1502685104226-ee32379fefbe?ixlib=rb-1.2.1&auto=format&fit=facearea&facepad=2&w=1024&h=1024&q=80",
		name: "John Doe",
		username: "@johndoe",
		content:
			'"ETMAR Logistics has transformed our supply chain management. Their efficiency and reliability are unmatched."',
	},
	{
		avatar:
			"https://images.unsplash.com/photo-1544005313-94ddf0286df2?ixlib=rb-1.2.1&auto=format&fit=facearea&facepad=2&w=1024&h=1024&q=80",
		name: "Jane Smith",
		username: "@janesmith",
		content:
			'"Working with ETMAR Logistics has been a game-changer for our business. Their team is professional and always delivers on time."',
	},
	{
		avatar:
			"https://images.unsplash.com/photo-1550525811-e5869dd03032?ixlib=rb-=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=1024&h=1024&q=80",
		name: "Alexa Goyette",
		username: "@alexagoyette",
		content:
			'"ETMAR Logistics has been an invaluable partner in our business. They have helped us streamline our logistics operations and have always been reliable in delivering our products on time."',
	},
];

const TestimonialsSection: Component<{}> = (props) => {
	return (
		<section class="py-12 rounded-md px-4 w-full flex flex-col gap-5">
			<div class="text-center">
				<span class="text-primary">Testimonials</span>
				<h2 class="heading-2">
					We have worked with thousands of amazing people
				</h2>
			</div>
			<div class="flex flex-col desktop:items-center">
				<Card class="desktop:w-1/2 shadow-lg intersect:animate-fade-up animate-delay-300 intersect-once">
					<CardContent>
						<blockquote class="blockquote desktop:heading-4 desktop:font-light">
							{TESTIMONAILS[0].content}
						</blockquote>
					</CardContent>
					<CardFooter>
						<div class="flex flex-row items-center gap-5">
							<Avatar>
								<AvatarImage src={TESTIMONAILS[0].avatar} />
							</Avatar>
							<div>
								<CardTitle>{TESTIMONAILS[0].name}</CardTitle>
								<CardDescription>{TESTIMONAILS[0].username}</CardDescription>
							</div>
						</div>
					</CardFooter>
				</Card>
			</div>
			<div class="grid md:grid-cols-1 lg:grid-cols-3 gap-5">
				<For each={TESTIMONAILS.splice(1)}>
					{(testimonial) => (
						<Card class="flex flex-col justify-end shadow-lg intersect:animate-fade-up animate-delay-300 intersect-once">
							<CardContent>
								<blockquote class="blockquote desktop:heading-4 desktop:font-light">
									{testimonial.content}
								</blockquote>
							</CardContent>
							<CardFooter>
								<div class="flex flex-row items-center gap-5">
									<Avatar>
										<AvatarImage src={testimonial.avatar} />
									</Avatar>
									<div>
										<CardTitle>{testimonial.name}</CardTitle>
										<CardDescription>{testimonial.username}</CardDescription>
									</div>
								</div>
							</CardFooter>
						</Card>
					)}
				</For>
			</div>
		</section>
	);
};

export default TestimonialsSection;
