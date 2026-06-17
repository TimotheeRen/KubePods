import { Button } from "@/components/ui/button";
import { NavMenu } from "@/components/nav-menu";
import { NavigationSheet } from "@/components/navigation-sheet";
import logo from "@/assets/logo.png"
import { AnimatedThemeToggler } from "./ui/animated-theme-toggler";
import RegisterDialog from "./registerDialog";
import LoginDialog from "./loginDialog";

const Navbar = () => {
  return (
    <nav className="h-16 border-b bg-background/20 backdrop-blur-md fixed w-full z-10">
      <div className="mx-auto flex h-full max-w-(--breakpoint-xl) items-center justify-between px-4 sm:px-6 lg:px-8">
        <div className="flex items-center gap-12">
          <a href="#" className="flex gap-1">
            <img src={logo} alt="Logo" className="w-7" />
            <span className="text-lg">KubePods</span>
          </a>

          {/* Desktop Menu */}
          <NavMenu className="hidden md:block" />
        </div>

        <div className="flex items-center gap-3">
          <LoginDialog>
            <Button className="hidden sm:inline-flex" variant="outline">
              Sign In
            </Button>
          </LoginDialog>
          <RegisterDialog>
            <Button>Sign Up</Button>
          </RegisterDialog>
          <Button size="icon" variant="outline">
            <AnimatedThemeToggler />
          </Button>

          {/* Mobile Menu */}
          <div className="md:hidden">
            <NavigationSheet />
          </div>
        </div>
      </div>
    </nav>
  );
};

export default Navbar;
