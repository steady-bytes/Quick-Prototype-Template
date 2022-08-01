import './App.css';
import { Link, Routes, Route } from "react-router-dom";

import {Landing} from "./pages";

/// application controllers
import { usePages } from "./controllers";

function App() {
  const pages = usePages({});
  console.log("error: ", pages.error)
  console.log(pages.data)

  if (typeof pages === 'undefined') {
    console.log("undefined data")
  }

  // iterate over the pages, and render the routes
//   {
//     "front_end_metadata_pages": [
//         {
//             "id": 2,
//             "name": "login",
//             "path": "/login",
//             "version": "1.0.0",
//             "created_at": "2022-07-30T16:54:23.393885+00:00",
//             "updated_at": "2022-07-30T16:54:23.393885+00:00"
//         },
//         {
//             "id": 1,
//             "name": "landing ",
//             "path": "/landing",
//             "version": "1.0.0",
//             "created_at": "2022-07-16T18:49:29.053955+00:00",
//             "updated_at": "2022-07-30T18:03:06.025355+00:00"
//         }
//     ]
// }

  return (
    <>
    <Routes>
      <Route path="/landing" element={<Landing />} />
    </Routes> 
    </>
  );
}

function Links() {
  // {pages != "undefined" && pages.data.map((page) => (
  //     <Link
  //       style={{ display: "block", margin: "1rem 0" }}
  //       to={`${page.route}`}
  //       key={page.route}
  //     >
  //       {page.name}
  //     </Link>
  //   ))}
}

export default App;