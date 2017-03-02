import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { HttpModule } from '@angular/http';

import { AppComponent } from './app.component';
import { FileListComponent } from './file-list/file-list.component';
import { FileListItemComponent } from './file-list/file-list-item/file-list-item.component';
import { NavBarComponent } from './file-list/nav-bar/nav-bar.component';

@NgModule({
  declarations: [
    AppComponent,
    FileListComponent,
    FileListItemComponent,
    NavBarComponent
  ],
  imports: [
    BrowserModule,
    FormsModule,
    HttpModule
  ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule { }
