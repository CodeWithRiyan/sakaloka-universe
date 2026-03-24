"use client"

import { FcGoogle } from "react-icons/fc"

import { Button } from "@/components/ui/button"
// import { useSearchParams } from "react-router";
// import { DEFAULT_LOGIN_REDIRECT_URL } from '@/constants';

export const Social = () => {
  // const [searchParams] = useSearchParams();
  // const callbackUrl = searchParams.get("callbackUrl");

  // const onClick = (provider: "google") => {
  //   signIn(provider, {
  //     callbackUrl: callbackUrl || DEFAULT_LOGIN_REDIRECT_URL,
  //   });
  // }

  return (
    <div className="flex w-full flex-col items-center gap-2">
      <div className="mb-4 flex w-full items-center justify-center">
        <div className="h-[1px] w-full bg-blue-400" />
        <p className="m-0 inline-flex justify-center px-4 py-2 text-base text-blue-400">
          atau
        </p>
        <div className="h-[1px] w-full bg-blue-400" />
      </div>
      <Button
        size="lg"
        className="w-full space-x-4"
        variant="outline"
        // onClick={() => onClick("google")}
      >
        <FcGoogle className="h-5 w-5" />
        <span>Lanjutkan dengan Google</span>
      </Button>
    </div>
  )
}
