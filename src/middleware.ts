import { invoke } from "@tauri-apps/api/core";

export const authentificator = async (
	_to: object,
	from: { path: string },
	next: Function,
) => {
	const redirectToLogin = () => next({ path: "/" });
	const redirectToApp = () => next({ path: "/conversation/" });
	const proceed = () => next();

	try {
		await invoke("get_user");

		if (from.path === "/") {
			redirectToApp();
		} else {
			proceed();
		}
	} catch (error) {
		if (from.path === "/") {
			proceed();
		} else {
			redirectToLogin();
		}
	}
};
