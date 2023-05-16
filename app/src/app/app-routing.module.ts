import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { VulnsComponent } from './vulns/vulns.component';

const routes: Routes = [
  { path: 'vulns', component: VulnsComponent },
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
