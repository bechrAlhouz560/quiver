import { Card, CardContent, CardDescription, CardTitle } from "../ui/card";

export default function WorkspaceCard() {
  return (
    <Card className="cursor-pointer hover:opacity-55 transition-opacity">
      <CardContent className="flex flex-col gap-2">
        <CardTitle className="text-lg font-bold">Workspace Name</CardTitle>
        <CardDescription>
          Lorem ipsum dolor sit amet consectetur adipisicing elit. Velit ex
          repellat tenetur, error labore maiores nostrum nulla dicta sed,
          voluptatem earum ipsa enim fugit dignissimos sint aliquam debitis, ut
          aut.
        </CardDescription>
      </CardContent>
    </Card>
  );
}
