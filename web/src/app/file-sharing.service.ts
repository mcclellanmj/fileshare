import { Injectable } from '@angular/core';
import { Http, URLSearchParams, RequestOptions } from '@angular/http';

import 'rxjs/add/operator/toPromise';

@Injectable()
export class FileSharingService {
  private readonly fileListUrl: string = '/view';

  constructor(private http: Http) { }

  getFiles(directory: string): Promise<File[]> {
    const params: URLSearchParams = new URLSearchParams();
    params.set("folder_path", directory);

    const options: RequestOptions = new RequestOptions();
    options.search = params;

    return this.http.get(this.fileListUrl, options)
               .toPromise()
               .then(response => {
                 return response.json() as File[];
               })
               .catch(this.handleError);
  }

  private handleError(error: any): Promise<any> {
    console.error('error during http', error);
    return Promise.reject(error.message || error);
  }
}
