import { Input } from "../ui/input";

export default function TopBar() {
  return (
    <div className="flex py-2 w-full shrink-0 items-center border-b px-4">
      <span className="font-bold text-primary">Quiver</span>
      <div className="flex-1 px-12">Hell</div>
      <Input placeholder="Search Here" className="w-fit" />
    </div>
  );
}
