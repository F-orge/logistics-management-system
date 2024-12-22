import { createAsync } from "@solidjs/router";
import { Suspense, type Component } from "solid-js";
import { HealthCheckClient } from "./lib/protoc/health_check.client";
import { transport } from "./entry-client";

const StatusPage: Component<{}> = (props) => {
	const healthService = new HealthCheckClient(transport);

	const serverHealth = createAsync(() => healthService.check({}).response);

	return (
		<Suspense fallback={<div>Loading...</div>}>
			{serverHealth()?.message}
		</Suspense>
	);
};

export default StatusPage;
