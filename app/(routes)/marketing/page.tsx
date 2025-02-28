import LandingSection from "./landing";
import ServicesSection from "./services";
import TrustedBySection from "./trusted-by";

export default function Page() {
  return (
    <article>
      <div>
        <LandingSection />
      </div>
      <div className="bg-neutral-800">
        <TrustedBySection />
      </div>
      <div className="">
        <ServicesSection />
      </div>
    </article>
  );
}
