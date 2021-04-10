// use yew::prelude::*;

// pub struct Model {
//     link: ComponentLink<Self>,
// }

// pub enum Msg {}

// impl Component for Model {
//     type Message = Msg;
//     type Properties = ();
//     fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
//         Self { link }
//     }

//     fn update(&mut self, _msg: Self::Message) -> ShouldRender {
//         true
//     }

//     fn change(&mut self, _props: Self::Properties) -> ShouldRender {
//         false
//     }

//     fn view(&self) -> Html {
//         html! {
//             <div class="shadow overflow-hidden border-b border-gray-200 sm:rounded-lg">
//             <table class="min-w-full divide-y divide-gray-200">
//               <thead class="bg-gray-50">
//                 <tr>
//                   <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
//                     {"Artist"}
//                   </th>
//                   <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
//                     {"Track"}
//                   </th>
//                   <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
//                     {"Length"}
//                   </th>
//                   <th scope="col" class="relative px-6 py-3">
//                     <span class="sr-only">{"Queue"}</span>
//                   </th>
//                 </tr>
//               </thead>
//               <tbody>
//                 <tr class="bg-white">
//                   <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
//                     {"Steppenwolf"}
//                   </td>
//                   <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
//                     {"Born to be wild"}
//                   </td>
//                   <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
//                     {"2:34"}
//                   </td>
//                   <td class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
//                     <a href="#" class="text-indigo-600 hover:text-indigo-900">{"Queue"}</a>
//                   </td>
//                 </tr>

//                 <tr class="bg-gray-50">
//                   <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
//                     {"Cody Fisher"}
//                   </td>
//                   <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
//                     {"Product Directives Officer"}
//                   </td>
//                   <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
//                     {"cody.fisher@example.com"}
//                   </td>
//                   <td class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
//                     <a href="#" class="text-indigo-600 hover:text-indigo-900">{"Queue"}</a>
//                   </td>
//                 </tr>
//               </tbody>
//             </table>
//           </div>
//         }
//     }
// }
