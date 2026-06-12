import { Button } from "@/components/ui/button";
import { createFileRoute, useRouter } from "@tanstack/react-router";

export const Route = createFileRoute("/dashboard/")({
  component: RouteComponent,
});

function RouteComponent() {
  const router = useRouter();
  return (
    <div className="flex-1">
      <Button
        onClick={() =>
          router.navigate({
            href: "workspaces",
          })
        }
      >
        Go Back
      </Button>
    </div>
  );
}
