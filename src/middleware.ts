import { invoke } from "@tauri-apps/api/core";

export const authentificator = async (
	_to: object,
	_from: { path: string },
	next: Function,
) => {
	const redirectToLogin = () => next({ path: "/" });
	const proceed = () => next();

	try {
		await invoke("get_user");
		proceed();
	} catch (error) {
		redirectToLogin();
	}
};

export const redirectIfConnected = async (
	_to: object,
	_from: { path: string },
	next: Function,
) => {
	const redirectToApp = () => next({ path: "/conversation/" });
	const proceed = () => next();

	try {
		await invoke("get_user");
		redirectToApp();
	} catch (error) {
		proceed();
	}
};
