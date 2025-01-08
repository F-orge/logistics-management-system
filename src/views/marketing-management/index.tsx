import type { Component } from "solid-js";
import { showToast } from "../components/ui/toast";
import { lazy } from "solid-js";
import TrustedByCompaniesSection from "~/features/marketing/trusted-by-companies";

const HeroSection = lazy(() => import("~/features/marketing/hero"));
const ServicesSection = lazy(() => import("~/features/marketing/services"));
const CookieConsent = lazy(() => import("~/features/marketing/cookie-consent"));
const Footer = lazy(() => import("~/features/marketing/footer"));
const TestimonialsSection = lazy(
	() => import("~/features/marketing/testimonials"),
);
const Header = lazy(() => import("~/features/marketing/header"));
const ContactSection = lazy(() => import("~/features/marketing/contact"));

const MarketingHomePage: Component<{}> = (props) => {
	if (!window.document.cookie.includes("cookie-accepted=true")) {
		console.log(window.document.cookie);
		showToast({
			title: "Accept Cookies",
			description: <CookieConsent />,
			duration: 10000,
		});
	}

	return (
		<main class="h-screen max-h-screen container mx-auto max-w-[1920px] ">
			<Header />
			<article>
				<HeroSection />
				<TrustedByCompaniesSection />
				<ServicesSection />
				<TestimonialsSection />
				<ContactSection />
			</article>
			<Footer />
		</main>
	);
};

export default MarketingHomePage;
